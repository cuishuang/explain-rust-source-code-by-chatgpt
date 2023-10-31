# File: rust-analyzer/crates/mbe/src/token_map.rs

在rust-analyzer仓库中，rust-analyzer/crates/mbe/src/token_map.rs文件是与语法解析和生成相关的代码文件。

该文件中定义了两个结构体TokenMap和TokenMapBuilder，以及一个枚举类型TokenTextRange。

1. TokenMap结构体：TokenMap是用于映射文本和语法树令牌之间的对应关系的结构体。它提供了根据文本位置查找对应令牌的能力。TokenMap内部维护了一个Vec<TokenEntry>，每个TokenEntry记录了令牌的文本范围、语法树节点和相关信息。

2. TokenMapBuilder结构体：TokenMapBuilder是用于构建TokenMap的结构体。它提供了一系列方法来添加和构建令牌映射表。根据输入的语法树节点，TokenMapBuilder会将对应的令牌文本范围添加到TokenMap中。

3. TokenTextRange枚举：TokenTextRange是语法树令牌的文本范围枚举类型。它包含三个变体：
    - Subtree：表示来自语法树中的一个子树的文本范围。
    - SingleToken：表示语法树中的一个单独令牌的文本范围。
    - DelimiterPair：表示语法树中的一个分隔符对的文本范围。

TokenTextRange枚举主要用于描述令牌的文本范围，被TokenMap用于映射和索引令牌的位置信息。

总之，rust-analyzer/crates/mbe/src/token_map.rs文件中的TokenMap结构体和TokenMapBuilder结构体用于记录和映射文本和语法树令牌的对应关系，而TokenTextRange枚举用于描述令牌的文本范围。这些结构体和枚举类型在语法解析和生成过程中起到了至关重要的作用。

