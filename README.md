# Compiler Assingment 

## Name Nishanth 
## Date Nov 17 2022

### Moto
> To create recursive descent parser for grammer 
-----
> *A  -> B C*
> 
> *B  -> B! | D*
> 
> *C  -> *A | epsilon*
> 
> *D  -> n  | (A)*
----
> After Resolving left recursive conflict 
> 
> *A  -> B C*
> 
> *B  -> D B`*
> 
> 
> *B' -> !B' | epsilon*
> 
> *C  -> *A | epsilon*
> 
> *D  -> n  | (A)*
----