# b树算法
假设这是一棵m阶的b树
1. 它的每个节点不能多于m-1个元素，非根节点也不能少于m/2-1个元素
2. 每个节点的元素按照大小排序，它左子树的元素都小于这个元素，右子树一定大于它
3. 根节点到所有叶子节点的长度相同

## b树的插入
1. 找到相应的叶子节点插入
2. 如果此时叶子节点的元素数量小于等于m-1，则结束
3. 反之，以这个节点中间元素为中心分裂成两个节点，中间元素插入到上一层节点中。若上一层节点元素数量大于等于m-1则，重复这个步骤

## b树的删除
1. 找到相应的叶子节点中的元素删除
2. 若此时叶子节点的元素数量大于等于m/2-1，结束
3. 反之，如果兄弟节点的元素数量*大于*m/2-1，则将父节点中的指向这个节点的元素下移，兄弟节点中一个元素上移。
4. 如果兄弟也没有大于m/2-1个元素，将父节点中指向这个节点的元素下移，并和兄弟节点以及该节点合并。

# b+树算法
假设这是一棵m阶的b+树
1. 它的每个节点不能多于m-1个元素，非根节点也不能少于m/2-1个元素
2. 每个节点的元素按照大小排序，它左子树的元素都小于这个元素，右子树一定大于它
3. 根节点到所有叶子节点的长度相同
4. 每个内部节点不存储元素，只存储搜索索引，只有叶子节点存储元素
5. 叶子节点之间有双向指针

## b+树插入
1. 找到相应的叶子节点，插入。如果叶子节点的元素小于等于m-1，则结束。
2. 反之，将这个节点分裂成两个，中间的元素*复制*到父节点中
3. 对于父节点同理，若插入之后索引数目大于m-1时分裂，但分裂之后向上移动中间的索引，不是*复制*

# b+树删除
1. 如果删除之后，叶子节点的元素个数大于等于m/2-1则结束
2. 反之，如果兄弟节点的元素个数大于m/2-1，则向兄弟借一个元素，这个元素*复制*到父节点对应索引位置
3. 如果兄弟节点元素个数小于等于m/2-1，则两个节点合并，删除父节点相应的索引。如果父节点的索引数量大于等于m/2-1，则结束。
4. 反之，如果父节点的兄弟节点的索引数量大于m/2-1，则父节点的父节点对应的索引下移到父节点，父节点的兄弟节点对应的索引上移到父节点的父节点
5. 如果父节点的兄弟节点小于等于m/2-1，则二者合并，父节点的父节点对应元素下移，进入新的合并节点。继续讨论父节点的父节点（重复4）