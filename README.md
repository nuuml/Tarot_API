# Rust Tarot API Server

Simple server to allow consumers to retrieve Tarot Card draws. 

### Current features:

- `{APIRoot}/draw` - will draw a single card from a standard 78 card Tarot deck
  - each draw will generate a new deck
- `{APIRoot}/customDraw` - allows custom patterns
  - options
    - number drawn
    - only major arcana
    - pattern of draw including skips
- authentication
- logging

### Roadmap:

- persistent auth keys

### About
This project is primarily for my own edification. I am learning Rust through doing several small projects in order to improve my skillsets. 

In recent years I have been focused on front-end technologies, so learning a modern "low-level" is very exciting to me. The last time I focused on low level languages was learning C in college.
