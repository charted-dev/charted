# Garbage Collection
**Garbage Collection** is a spawned background task when the API service starts (or as a long-lived process with `charted gc --long-lived`) which will collect entities and check if they need to be thrown away into the void.
