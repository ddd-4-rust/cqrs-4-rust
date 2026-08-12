# cqrs-4-rust 序列化设计

- 日期: 2026-07-23
- 范围: `crates/serialization/{serde,jaxb,jsonb}/` (22 + 15 + 22 = 59 个迁移职责)
- 状态: 三线生产职责与测试职责到位；JAXB 黄金 XML 样本最终验收仍 defer

## 1. 背景

Java 版 `cqrs-4-java` 提供三套序列化：Jackson（默认 JSON）、JAXB（XML 兼容线）、JSON-B（Jakarta JSON Binding 兼容线）。三套分别对应 22 / 15 / 22 个迁移职责，共 59 个 `.rs` 文件。

Rust 端：

- `serde` 取代 Jackson：覆盖 JSON 主线；
- `jaxb` 保持 XML 兼容线（不把 JSON 类型别名伪装成 JAXB）；
- `jsonb` 保持 JSON-B 兼容线，使用 `inventory` 编译期注册替代 Jandex 运行时扫描。

## 2. 目标

1. 三线都使用同一份 `ResultType` 线协议（`OK` / `WARNING` / `ERROR`）。
2. JSON 兼容字段固定为 `type/code/message/data-class/data-element/<dynamic-element>`。
3. JAXB 兼容线走 XML serializer，不复用 JSON 类型别名。
4. JSON-B 注册使用 `inventory::submit!` 编译期注册。
5. 每个 Java 测试文件 → 唯一同名 snake_case Rust 测试文件。

## 3. 非目标

- 不引入 Cap'n Proto / MessagePack / Protobuf（不在 140 个迁移职责内）。
- 不替换 `serde` 的核心行为（仅封装 + 协议字段）。

## 4. 模块布局

```
crates/serialization/
├── serde/                          # 源 jackson (22)
│   ├── src/
│   │   ├── abstract_aggregate_command.rs
│   │   ├── abstract_command.rs
│   │   ├── abstract_result.rs
│   │   ├── cqrs_4_serde_module.rs  # 源 Cqrs4JacksonModule
│   │   ├── data_result.rs
│   │   ├── data_result_deserializer.rs  # 源 DataResultJacksonDeserializer
│   │   ├── data_result_serializer.rs    # 源 DataResultJacksonSerializer
│   │   └── simple_result.rs
│   └── tests/  (14)
│       ├── a_created_event.rs / a_id.rs / b_id.rs / c_id.rs
│       ├── abstract_aggregate_command_test.rs / abstract_command_test.rs
│       ├── architecture_test.rs / base_test.rs
│       ├── data_result_deserializer_test.rs / data_result_test.rs
│       ├── invoice.rs / my_id_factory.rs / simple_result_test.rs
│       └── test_utils.rs
├── jaxb/                           # 源 jaxb (15)
│   ├── src/  (5)
│   │   ├── abstract_aggregate_command.rs
│   │   ├── abstract_command.rs
│   │   ├── abstract_result.rs
│   │   ├── data_result.rs
│   │   └── simple_result.rs
│   └── tests/  (10)
└── jsonb/                          # 源 jsonb (22)
    ├── src/  (8)
    │   ├── abstract_aggregate_command.rs
    │   ├── abstract_command.rs
    │   ├── abstract_result.rs
    │   ├── data_result.rs
    │   ├── data_result_jsonb_adapter.rs
    │   ├── jandex_jsonb_registry.rs
    │   ├── jsonb_registry.rs
    │   └── simple_result.rs
    └── tests/  (14)
```

## 5. 协议字段

JSON 输出（serde / jsonb）：

```json
{
  "type": "OK",
  "code": null,
  "message": null,
  "data-class": "com.example.Person",
  "data-element": { "id": "...", "name": "..." },
  "<dynamic-element>": ...
}
```

`ResultType` 字符串固定（与 Java 兼容）：

| Rust | JSON 值 |
|---|---|
| `ResultType::Ok` | `"OK"` |
| `ResultType::Warning` | `"WARNING"` |
| `ResultType::Error` | `"ERROR"` |

`code` 与 `message` 在 `Ok` 时为 `null`。

XML 输出（jaxb）：

```xml
<result>
  <type>OK</type>
  <code xsi:nil="true"/>
  <message xsi:nil="true"/>
  <data class="com.example.Person">...</data>
</result>
```

> 注：Java JAXB XML 元素命名约定为小驼峰或带前缀；Rust 端用 `quick-xml` + 自定义 serializer，禁止复用 JSON 的 kebab-case 字段名。

## 6. 注册机制

JSON-B 注册（编译期）：

```rust
#[derive(Serialize, Deserialize)]
pub struct PersonCreatedEvent { /* ... */ }

inventory::submit! {
    JsonbRegistryEntry {
        event_type: "PersonCreatedEvent",
        adapter_factory: || Box::new(PersonCreatedEventAdapter),
    }
}
```

测试在 `tests/architecture_test.rs` 中验证所有 `submit!` 注册项符合 `JsonbRegistryEntry` 形状。

JAXB 注册：

```rust
inventory::submit! {
    JaxbRegistryEntry {
        event_type: "PersonCreatedEvent",
        xml_root: "person-created-event",
    }
}
```

## 7. 与 Java 的差异

| 差异 | 解释 |
|---|---|
| Jackson → Serde | Java Jackson 用 `@JsonCreator` / `@JsonProperty`；Rust 用 `#[derive(Serialize, Deserialize)]` + `#[serde(rename)]` |
| `data-class` 字段名带连字符 | 必须保持，Java 用 `@JsonProperty("data-class")` |
| Jandex → `inventory` | Java 用 Jandex 运行时扫描；Rust 用 `inventory` 编译期注册避免运行时反射 |
| JAXB XML 元素名 kebab-case | 与 Java camelCase 不一致；用 `#[serde(rename = "kebab-case")]` 等价机制或 `quick-xml` 自定义 |

## 8. 测试覆盖

每个生产文件对应同名测试文件（除 `cqrs_4_serde_module.rs` 等聚合模块），覆盖：

- JSON 编解码与字段协议（type/code/message/data-class/data-element）
- JAXB XML 编解码
- JSON-B 编译期注册扫描
- 错误分支（`ResultType::Error` 时 data 为 null）
- 未知事件类型返回错误

## 9. 完成定义

- 22 + 15 + 22 = 59 个 `.rs` 文件齐全
- `cargo test --workspace --all-features` 通过
- JAXB 黄金 XML 样本（来自 Java `JaxbTest` fixture）回归通过
- 禁止用 JSON 类型别名伪装 XML（CI lint 校验字段命名一致性）