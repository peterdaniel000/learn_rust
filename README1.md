      OWNERSHIP 
Is the managing of memory storage
STACK AND THE HEAP

A STACK memory allocation scheme is a linear memory allocation scheme where data are arrange in an orderly manner, where the the last placed is the first to be called, adding data is called pushing while removing data is called popping off

A HEAP memory allocation scheme is a disorder memory allocation scheme where data are placed anywhere there exist a free space is naturally slower than stack  because to access a data in a memory you have to follow a pointer to get there

OWNERSHIP: there are rules that govern ownership 
1. Each value in rust has a variable that is called its owner
2. There can be only one owner at a time 
3. When the owner goes out of scope, the value will be dropped