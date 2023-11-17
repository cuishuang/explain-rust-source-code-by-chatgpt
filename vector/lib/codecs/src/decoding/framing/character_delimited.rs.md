# File: vector/lib/codecs/src/decoding/framing/character_delimited.rs

vector/lib/codecs/src/decoding/framing/character_delimited.rs

这个文件是Rust生态中的vector项目中的vector-1.1.1/src/vector/src/codecs/src/decoding/framing目录下的一个文件，它实现了字符分割解码器的功能。

CharacterDelimitedDecoderConfig: 这个结构体定义了用于配置字符分割解码器的参数。它可以设置字符分隔符、编码方式、最大消息长度等属性。

CharacterDelimitedDecoderOptions: 这个结构体定义了字符分割解码器的选项。它包含了设置字符分隔符、编码方式、最大消息长度等属性的方法，还提供了对配置参数的验证方法。

CharacterDelimitedDecoder: 这个结构体是实际的字符分割解码器，它使用字符分隔符将输入流切分为多个消息。它根据配置参数进行初始化，并提供了decode方法用于解码消息。decode方法接受一个输入字节流作为参数，并返回解码后的消息。

这些结构体一起实现了字符分割解码器的功能。通过配置参数配置解码器的行为，然后使用decode方法对输入流进行解码，将输入流切分为多个消息。这个解码器可以用于处理以字符分隔符分隔的消息流，提供了对输入流的解码功能。

