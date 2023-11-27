本章比较重要

特别是所有权移动问题

 所有权树不能半空 节点如果被移动 整棵树处于`uninitialized`状态
 
### 不能被copy的类型

As a rule of thumb, any type that needs to do something special when a value is drop‐ ped cannot be Copy: 
a Vec needs to free its elements,
a File needs to close its file han‐ dle, 
a MutexGuard needs to unlock its mutex, and so on. 
Bit-for-bit duplication of such types would leave it unclear which value was now responsible for the original’s resources.