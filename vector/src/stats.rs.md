# File: vector/src/stats.rs

vector/src/stats.rs这个文件用于统计计算相关的功能，包括指数加权移动平均值(Ewma)，方差和均值(MeanVariance)，以及均值(Mean)。下面分别介绍每个结构体的作用：

1. Ewma (Exponentially Weighted Moving Average，指数加权移动平均值)
   - Ewma结构体用于计算一组数据的指数加权移动平均值。
   - 对于给定的时间窗口长度，Ewma使用负指数衰减计算数据点的加权平均值，新数据点的权重由其时间戳与当前时间之间的差异决定。
   - 这个结构体还提供了更新移动平均值的方法和获取当前移动平均值的方法。

2. EwmaDefault
   - EwmaDefault结构体是Ewma的一个简化版本，可以通过指定半衰期而非时间窗口长度来计算指数加权移动平均值。

3. EwmaVar (Exponentially Weighted Moving Average Variance，指数加权移动平均方差)
   - EwmaVar结构体用于计算一组数据的指数加权移动平均方差。
   - 类似于Ewma结构体，EwmaVar结构体使用负指数衰减计算数据点的加权平均方差，新数据点的权重由其时间戳与当前时间之间的差异决定。
   - 这个结构体还提供了更新移动平均方差的方法和获取当前移动平均方差的方法。

4. MeanVariance
   - MeanVariance结构体用于计算一组数据的均值和方差。
   - 这个结构体提供了更新均值和方差的方法，以及获取当前均值和方差的方法。

5. Mean
   - Mean结构体用于计算一组数据的均值。
   - 这个结构体提供了更新均值的方法和获取当前均值的方法。

总之，这些结构体都是用于数据统计和计算相关的功能，帮助在Rust生态vector项目中对数据进行分析和计算。

