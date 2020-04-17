# LFS
1. inode记录data block的信息（地址、文件信息等，每个文件可能不止一个data block）

2. 用inode map来存放inode的信息（每个inode的地址和它的最新版本）

3. check region存放inode map的信息，位于磁盘的开头

4. 垃圾回收：每个data block都有一个summary block，记录它归属的文件inode以及文件中的位置。再通过imap记录的inode信息和inode中记录的文件地址就可以知道这是不是最新版本的data block。