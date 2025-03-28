# 日志

## 1.环境准备

### 1.1.1  
进入https://www.releases.ubuntu.com/22.04/  
下载ubuntu-22.04.5-desktop-amd64.iso  

### 1.1.2  
进入https://www.oracle.com/cn/virtualization/technologies/vm/downloads/virtualbox-downloads.html  
下载VirtualBox-7.1.6-167084-Win.exe  
下载Oracle_VirtualBox_Extension_Pack-7.1.6.vbox-extpack  
下载VBoxGuestAdditions_7.0.2.iso  

### 1.2.1  
安装virtualbox  
安装Oracle_VirtualBox_Extension_Pack-7.1.6.vbox-extpack  

### 1.2.2  
新建虚拟机ubuntu-22.04  
虚拟硬盘分配30GB，一次性分配  
安装虚拟机增强功能VBoxGuestAdditions  
配置虚拟机共享文件夹和网络（网络地址转换NAT模式）  

### 1.2.3  
虚拟机处理器分配2CPU  
启用PAE/NX  
启用嵌套VT-x/AMD-V  

## 2.选择并配置BaseOS

### 2.1.1  可选的BaseOs：  
C-based OS  
https://github.com/oscomp/RT-Thread  
Rust-based OS  
https://github.com/oscomp/ByteOS  
https://github.com/oscomp/DragonOS  
https://github.com/oscomp/asterinas  
https://github.com/oscomp/starry-next  

### 2.1.2  确定选择DragonOS  
​优势：  
​明确的技术目标：面向云计算场景，提供Linux二进制兼容性，符合未来操作系统发展趋势。  
文档完善：适合新手快速理解架构。   
​项目友好：提供CI工具链和现成测试框架，适合快速完成课程作业的“可验证优化”。  
​优化方向：  
增强实时性（如调度算法优化）。实现简单的优先级调度或CFS（公平调度）。  
扩展RISC-V架构支持或设备驱动（如VIRTIO网卡/块设备驱动，参考drivers/virtio目录）。  
实现更多POSIX接口以提升兼容性。适配更多Linux系统调用（当前约1/4，可逐步补全）。  

代码仓库：https://github.com/oscomp/DragonOS  
文档：https://docs.dragonos.org.cn/introduction/build_system.html  

### 2.2.1  下载DragonOS的源代码  
使用ssh克隆:  
git clone git@github.com:DragonOS-Community/DragonOS.git  
cd DragonOS  
使用镜像源更新子模块:  
make update-submodules-by-mirror  

### 2.2.2  使用一键初始化脚本进行安装  
/DragonOS/tools目录下执行:  
bash bootstrap.sh  

## 3.遇到的问题与解决方法  

### 3.1  虚拟机网络配置
问题：虚拟机无法连接github.com  
解决：手动修改DNS（下列二选一）  
  echo "nameserver 8.8.8.8" | sudo tee /etc/resolv.conf  
  echo "nameserver 223.5.5.5" | sudo tee /etc/resolv.conf  
然后更新软件源：  
  sudo apt update  
  sudo apt-get update  

### 3.2  虚拟硬盘空间  
问题：在安装DragonOS时虚拟硬盘空间不足（25G）  
解决：创建虚拟机时分配30G，一次性分配  
  这浪费了我很多时间，因为我安装DragonOS失败后，无法重新进入Ubuntu22.04系统，只显示“/dev/sda3:clean,418486/1605632 files, 6128935/6421504 blocks”，于是我只好删除虚拟机从头来过。

### 3.3  虚拟化技术
问题：无法运行DragonOS  
解决：在宿主机的BIOS中设置Intel Virtualization Technology启用，更新VirtualBox和扩展包Oracle_VirtualBox_Extension_Pack-7.1.6.vbox-extpack（必要）  
  宿主机cmd中执行：VBoxManage modifyvm "Ubuntu22.04" --nested-hw-virt on  
