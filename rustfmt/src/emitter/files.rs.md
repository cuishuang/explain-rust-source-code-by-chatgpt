# File: /Users/fliter/rust-contribute/rustfmt/src/emitter/files.rs

在Rust的rustfmt项目中，/Users/fliter/rust-contribute/rustfmt/src/emitter/files.rs这个文件的作用是在格式化后将修改的文件写回到磁盘上。

FilesEmitter这个文件定义了几个struct：FileRecord、CreatedFiles和FilesEmitter。

1. FileRecord：代表一个文件的记录，包含了文件的路径、内容和与之相关联的状态。它有两个可能的状态：
   - Parsed：表示文件已经通过解析器（parser）被解析，但还未格式化。
   - Formatted：表示文件已经格式化完成。

2. CreatedFiles：是一个HashMap，用于存储已经创建的文件的记录。它定义了添加文件记录、检查文件是否已经存在、获取文件的可变引用等操作。

3. FilesEmitter：是一个文件格式化的实际执行者，它使用上述的CreatedFiles来管理文件记录。它的主要作用是将格式化后的内容写回到磁盘上的文件中。FilesEmitter类中的方法包括：
   - `create_file()`：根据给定的文件路径创建一个新的文件，并将其添加到CreatedFiles中。
   - `add_diff()`：将格式化前和格式化后的内容之间的差异添加到文件记录中，以便后续写入文件时可以根据这个差异进行增量写入。
   - `overwrite_file()`：根据文件记录中的内容，将格式化后的内容写回到磁盘上的文件中，覆盖原有的文件内容。
   - `finalize()`：完成所有文件的写入操作，将所有的创建和覆盖文件的操作进行提交。

总之，FilesEmitter结构体及其相关方法是rustfmt项目中负责处理文件的格式化、创建和写入操作的核心组件。

