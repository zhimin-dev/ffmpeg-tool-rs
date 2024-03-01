# ffmpeg-tool-rs

一个关于ffmpeg的常用工具

## usage

### 多个视频合并成一个

比如当前有2个视频`IMG_1767.MOV`以及`IMG_1768.MOV`，需要将这2个视频合并成一个视频，
你需要指定`-r IMG_(.*).MOV`并且指定开始的id(`--reg-file-start=`)以及结束的id(`--reg-file-end=`)
则执行下面的命令即可,会得到一个`IMG_.MOV`的合并后的视频文件。

```
ffmpeg-tool-rs combine -r IMG_\(\.\*\).MOV --reg-file-start=1767 --reg-file-end=1768
```

当然也可以指定生成后的文件名，需要跟上`--target_file_name=your_filename.MOV`


### 下载视频

```
ffmpeg-tool-rs download --url=https://zmis.me/xxx.m3u8
```

### 截取视频

-i 需要截取的视频

-s 视频开始的秒数

-d 截取视频的时长

```
ffmpeg-tool-rs cut -i=/your/local/file.mp4 -s=5 -d=10
```
