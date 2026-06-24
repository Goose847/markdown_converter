# markdown_converter
This is another educational exercise to learn Rust. It is a basic tool that will convert simple markdown files into HTML. 
I will define usage as I build it out but I'll be using the README as a place to build out the spec. 

## Spec
Milestones to build out:
 - CLI - basic take input, define output.
 - Ingestion - Read markdown file into some data structure.
 - Conversion - sift through text and convert using some rules.
 - IO handling - load and save. 
 - Error handling - Build out an error enumerator.


 ### CLI
 Basic usage should look like: 
 ```sh
 markdown_converter doc.md -o doc.html
 ```

 Given a md doc like:
 ```markdown
 # Holiday Ideas
 
 ## Beach
  - Relaxing
  - Swimming
  - Beach town vibes
  - expensive
  
 ## Camping
  - No signal
  - Out in nature
  - Chilling round the fire
  - Difficult to keep dirt out of your tent
 ```

Would give:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
</head>
<body>
    <h1>Holiday Ideas</h1>
    
    <h2>Beach</h2>
    <ul>
        <li>Relaxing</li>
        <li>Swimming</li>
        <li>Beach town vibes</li>
        <li>expensive</li>
    </ul>
        
    <h2>Camping</h2>
    <ul>
        <li>No signal</li>
        <li>Out in nature</li>
        <li>Chilling round the fire</li>
        <li>Difficult to keep dirt out of your tent</li>       
    </ul>
</body>
```

 ### Ingestion and conversion:
 Previously I was thinking about building out a loader that would hold the whole md file in memory, convert it into an html format, and then save it with some to_string type function. I am now thinking of doing a write as you process, whereby we seek the end of a section (ordered list, unordered list) and load it into a buffer. This would allow some flexibility of being able to convert files of arbritrary size. 

So what I need is:
 - Buffer handler (reads in a syntactically complete piece of markdown)
 - Converter based on what it is (define below)
 - Writer, append to file 
 - EOF handler to end doc.

### Error handling
The types of errors I'll be looking out for will be:
 - File not found / file found but empty, file found but not an md file.
 - Syntax not recognized/incorrect.
