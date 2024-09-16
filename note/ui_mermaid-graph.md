### [mermaid habr](https://habr.com/ru/articles/652867/)

### Mermaid graphs

```mermaid
graph LR
    A0[Get a task] --> A1
    A1[Formalize] --> A2
    A2[Think] --> A
    A[Write Code] --> B{Works?}
    B -- Yes --> C[Great]
    B -- No --> D[ChatGPT]
    D --> A2
```