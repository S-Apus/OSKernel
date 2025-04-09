# 日志2

## 1. 扩展虚拟硬盘

### 1.1 问题

我希望本地编译DragonOS，写入磁盘镜像，并在QEMU中运行（make run）

但是虚拟硬盘空间不足了，所以需要扩展虚拟硬盘空间，但发现.vdi格式的虚拟硬盘不支持扩展

### 1.2 解决

搭建虚拟机共享文件夹Share，直接在Share中构建，相当于直接使用宿主机硬盘空间。

### 1.3 权限问题

操作系统不能编译，因为我没有共享文件夹的所有权

### 1.4 最终方案

使用.vdi虚拟硬盘，不要选择一次性分配，选择动态分配。

扩展分区： 
使用sudo gparted或者growpart（推荐）工具

#### 我又重头来过了，这又耽误了我很长时间！

## 2. 配置git仓库

### 2.1 SSH Key

生成公钥：
~~~
ssh-keygen -t ed25519 -C "2673516269@qq.com"
~~~

复制公钥：
~~~
cat ~/.ssh/id_ed25519.pub
~~~

添加公钥到github：  
https://github.com/settings/profile

启动 SSH 代理：
~~~
eval "$(ssh-agent -s)"
~~~

添加私钥到代理：
~~~
ssh-add ~/.ssh/id_ed25519
~~~

测试公钥：
~~~
ssh -T git@github.com
~~~

#### 如果报错“Connection refused”：  
在~/.ssh处执行：nano config  
输入：  
~~~
Host github.com
  Hostname ssh.github.com
  Port 443
  User git
~~~
保存

### 2.2 仓库配置

将仓库路径添加到 Git 的全局安全目录列表中：
~~~
git config --global --add safe.directory /media/sf_Share/DragonOS
~~~
