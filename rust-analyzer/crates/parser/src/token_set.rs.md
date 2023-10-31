# File: rust-analyzer/crates/parser/src/token_set.rs

在rust-analyzer的源代码中，rust-analyzer/crates/parser/src/token_set.rs文件的作用是定义了一个用于存储语法单元的集合的数据结构，即TokenSet。

TokenSet是由一组用于表示语法单元的token组成的集合。它采用了位向量（bit vector）的形式来表示集合的成员关系，使用u128作为底层数据类型。每个bit位代表一个语法单元，如果对应的bit位为1，则表示该语法单元属于集合中，否则不属于。

TokenSet结构体的几个主要功能如下：

1. `TokenSet::new()`：创建一个空的TokenSet。

2. `TokenSet::from_single(token: u16)`：创建包含指定单个token的TokenSet。

3. `TokenSet::from(tokens: &[u16])`：创建包含指定一组tokens的TokenSet。

4. `TokenSet::contains(&self, token: u16) -> bool`：判断TokenSet中是否包含指定的token。

5. `TokenSet::is_disjoint(&self, other: TokenSet) -> bool`：判断当前TokenSet与另一个TokenSet是否没有交集。

6. `TokenSet::union(&self, other: TokenSet) -> TokenSet`：计算当前TokenSet和另一个TokenSet的并集。

7. `TokenSet::intersection(&self, other: TokenSet) -> TokenSet`：计算当前TokenSet和另一个TokenSet的交集。

8. `TokenSet::subtract(&self, other: TokenSet) -> TokenSet`：计算当前TokenSet减去另一个TokenSet的差集。

这些功能可以用于对语法单元进行集合操作，例如判断某个语法单元是否属于集合、计算两个集合的交集、并集或差集等。

通过使用位向量来表示集合，TokenSet在空间复杂度上具有很高的效率。而u128作为底层数据类型，提供了128位的存储空间，足以容纳大量的语法单元。

