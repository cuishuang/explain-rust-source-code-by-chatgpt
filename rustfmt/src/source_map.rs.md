# File: /Users/fliter/rust-contribute/rustfmt/src/source_map.rs

/Users/fliter/rust-contribute/rustfmt/src/source_map.rs文件在Rust的rustfmt项目中的作用是实现源代码的位置相关的功能。该文件定义了SpanUtils和LineRangeUtils两个trait，提供了方便操作代码位置信息的方法。

SpanUtils trait提供了与代码范围相关的一些实用方法。它定义了从代码范围(span)获取行号、列号等信息的函数。这些函数使得可以准确地定位代码中的位置，便于进行代码格式化的处理。例如，可以使用函数`span_to_snippet`从代码范围中获取原始代码文本，并使用函数`offset`获取代码范围在整个文件中的偏移量。

LineRangeUtils trait提供了与代码行范围相关的实用方法。它定义了一些函数，用于获取代码范围内的行号列表，计算行数等。这些函数用于处理源代码中多行范围的操作，如解析代码中的注释、计算代码范围内有多少行等。

这两个trait共同提供了对代码位置和范围的处理和操作。它们在源代码格式化的过程中起到了重要作用，使得格式化引擎可以准确地定位代码的位置并对其进行处理。

