# Tensorflow 人脸抠图

| 序号 | 作者  | 版本   | 时间       |
| ---- | ----- | ------ | ---------- |
| 1    | Sunny | V0.1.0 | 2021-09-17 |




## 资料来源

<https://github.com/cetra3/mtcnn>

<https://github.com/blaueck/tf-mtcnn>

<https://github.com/kpzhang93/MTCNN_face_detection_alignment>



[Tensorflow Rust实战上篇](https://segmentfault.com/a/1190000018940111?utm_source=sf-similar-article)

[Tensorflow Rust实战下篇](https://segmentfault.com/a/1190000019616388)



## 环境

rustc 1.55.0

```toml
[dependencies]
tensorflow = { version = "0.17.0", features = ["tensorflow_gpu"] }
#没有gpu 直接用下面这句
# tensorflow = "0.17.0"
image = "0.23"
imageproc = "0.22"
structopt = "0.3"
```



## 编译

```shell
RUSTFLAGS="-Clink-arg=-Wl,-rpath,./" cargo build --release
```

拷贝可运行的环境

```shell
cp ./target/release/build/tensorflow-sys-da0aee35ae4b8faa/out/libtensorflow* .

cp target/release/mtcnn . 
```

执行

```shell
./mtcnn images/00.jpeg 00.jpeg
```





