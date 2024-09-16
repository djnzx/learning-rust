### Plant UML Diagrams

```plantuml
Alice -> Bob: Authentication Request
Bob --> Alice: Authentication Response

Jim -> Bob: Another authentication Request
Jim <-- Bob: Another authentication Response
```

```plantuml
'type       name           alias
participant Participant1 as P
actor       Actor2       as A
boundary    Boundary3    as Foo2
control     Control4     as Foo3
entity      Entity5      as Foo4
database    Database6    as Foo5
collections Collections7 as Foo6
queue       Queue8       as Q
P -> A : To actor 
P -> Foo2 : To boundary
P -> Foo3 : To control
P -> Foo4 : To entity
P -> Foo5 : To database
P -> Foo6 : To collections
P -> Q: To queue
```
