# File: /Users/fliter/rust-contribute/rustfmt/src/emitter/modified_lines.rs

在Rust的rustfmt项目的源代码中，/Users/fliter/rust-contribute/rustfmt/src/emitter/modified_lines.rs这个文件的作用是定义了用于处理、记录和格式化修改过的代码行的结构和方法。

该文件中主要定义了三个结构体：ModifiedLinesEmitter、Modification、ModificationType。

1. ModifiedLinesEmitter：这个结构体实现了`Emitter` trait，用于处理和记录修改过的代码行。它包含以下字段和方法：
   - `file_name: &'static str`：表示文件名，用于标识当前的修改行发生在哪个文件中。
   - `modified_lines: Vec<Modification>`：存储了所有修改过的代码行的信息，以`Modification`结构体的形式保存。
   - `emit_header()`：输出文件头部信息。
   - `emit_line()`：输出一行修改过的代码。
   - `modify_lines()`：记录修改的代码行信息。
   - `take_modified_lines()`：获取所有修改过的代码行信息。
   - `maybe_emit_modified_lines()`：根据`format_style`配置，决定是否输出修改过的代码行信息。

2. Modification：这个结构体用于表示一个修改了的代码行的信息。它包含以下字段和方法：
   - `line_number: usize`：表示代码行在文件中的行号。
   - `original_code: String`：表示原始的代码行内容。
   - `modified_code: String`：表示修改后的代码行内容。
   - `modification_type: ModificationType`：表示修改的类型（添加、删除或修改）。
   - `is_empty()`：判断该修改行是否为空行。

3. ModificationType：这个枚举类型表示代码行的修改类型，包括以下几种：
   - `Added`：添加了新的代码行。
   - `Deleted`：删除了原始的代码行。
   - `Modified`：修改了原始的代码行，使之变为新的代码行。

通过这些结构体和方法，ModifiedLinesEmitter可以对修改过的代码行进行处理、记录和格式化，并最终输出相应的修改行信息。这在代码格式化工具中是非常有用的，可以让开发者了解到代码格式化对原始代码所作出的修改。

