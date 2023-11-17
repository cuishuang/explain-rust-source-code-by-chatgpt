# File: vector/src/transforms/sample.rs

在Rust生态的vector项目中，vector/src/transforms/sample.rs文件的作用是实现样本抽样的功能。

SampleConfig结构体用于配置样本抽样的行为。它包含以下字段：

- `kind`: 样本抽样的类型，可以是"random"（随机抽样）或"fixed"（固定抽样）。
- `probability`: 当kind设置为"random"时，probability表示抽样的概率。当kind设置为"fixed"时，probability表示每个输入事件被选中的频率。
- `seed`: 随机数生成器的种子，用于复现相同的随机序列。

该结构体的实例通过配置文件或运行时参数进行配置。

Sample结构体是样本抽样的转换器，实现了Transform trait。它提供以下功能：

- `new`: 创建一个新的样本抽样转换器。
- `transform`: 实现样本抽样的转换逻辑。根据配置文件中的probability来判断每个事件是否要被选中。
- `to_string`: 将样本抽样的配置转换为字符串表示。

样本抽样的逻辑如下：
- 当kind设置为"random"时，每个事件独立地根据probability的概率被选中。
- 当kind设置为"fixed"时，每个事件根据probability的值被选中的频率选中。

可以通过创建SampleConfig结构体的实例来配置样本抽样的行为，然后使用Sample结构体的实例来进行样本抽样的转换操作。

