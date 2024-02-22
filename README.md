# sort_text_international_rust_wasm_pwa

[//]: # (auto_cargo_toml_to_md start)

**04. Tutorial for Coding simple PWA in Rust (sort_text_international_rust_wasm_pwa) (2022-07)**  
***version: 2022.717.1738 date: 2022-07-17 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/sort_text_international_rust_wasm_pwa)***  

[//]: # (auto_cargo_toml_to_md end)

 ![maintained](https://img.shields.io/badge/maintained-green)
 ![ready_for_use](https://img.shields.io/badge/ready_for_use-green)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-159-green.svg)](https://github.com/bestia-dev/sort_text_international_rust_wasm_pwa/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-101-blue.svg)](https://github.com/bestia-dev/sort_text_international_rust_wasm_pwa/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-23-purple.svg)](https://github.com/bestia-dev/sort_text_international_rust_wasm_pwa/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/sort_text_international_rust_wasm_pwa/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/sort_text_international_rust_wasm_pwa/)

[//]: # (auto_lines_of_code end)

 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/sort_text_international_rust_wasm_pwa/blob/master/LICENSE)
 [![Rust](https://github.com/bestia-dev/sort_text_international_rust_wasm_pwa/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/sort_text_international_rust_wasm_pwa/)
 ![sort_text_international_rust_wasm_pwa](https://bestia.dev/webpage_hit_counter/get_svg_image/194693097.svg)

Hashtags: #rustlang #tutorial #pwa #wasm #webassembly  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Try it

<https://bestia.dev/sort_text_international_rust_wasm_pwa/>  

This project has also a youtube video tutorial. Watch it:
<!-- markdownlint-disable MD033 -->
[<img src="https://bestia.dev/youtube/sort_text_international_rust_wasm_pwa.jpg" width="400px">](https://bestia.dev/youtube/sort_text_international_rust_wasm_pwa.html)
<!-- markdownlint-enable MD033 -->

## Development

My development environment is thoroughly explained in my previous projects with youtube video tutorials:  
[1. Linux everywhere! Install wsl2 and debian11 on win10 (win10_wsl2_debian11) (2022-03)](https://github.com/bestia-dev/win10_wsl2_debian11)  
[2. Rust: Hack Without Fear ! Rust Development Environment in Docker Container. (docker_rust_development) (2022-03)](https://github.com/bestia-dev/docker_rust_development)  
[3. Coding a Rust client CLI for PlantUml server (rust_plantuml_client) (2022-04)](https://github.com/bestia-dev/rust_plantuml_client)  

Use `cargo-auto` to automate development tasks: `cargo install cargo-auto`.  
Then inside the Rust project folder run `cargo auto` for the instructions.
PWA files MUST be served by a web server. We will use the most simple development web server:  
`cargo install basic-http-server`.  
Open a new terminal window in VSCode and go to the web server folder and run the server:  
`cd ~/rustprojects/sort_text_international_rust_wasm_pwa/web_server_folder; basic-http-server`  
Inside VSCode add the port 4000 for forwarding out of the docker container.
Open the browser in Win10 on:  
<http://127.0.0.1:4000/sort_text_international_rust_wasm_pwa/>  

## Alphabetical sorting (collation)

I will use the javascript Intl Collator object to sort text for different languages.
Rust does not have yet a stable collation library.  
Rust (wasm) and javascript can work very well together with web_sys and js_sys crates using wasm-bindgen.  

## Template

This project was made from the template of a simple Rust Wasm PWA.  
It is created with this PWA utility:  
<https://bestia.dev/bestia_dev_new_rust_wasm_pwa>

## Video subtitles

Welcome to bestia.dev !
Learning Rust and Wasm programming and having fun.
I just love  programming !

In my first video tutorial, we set up WSL 2 (Windows Subsystem for Linux) with Debian 11 on Windows 10.
<https://bestia.dev/youtube/win10_wsl2_debian11.html>
In the second video, we created a Docker container with a complete Rust development environment to use with VS Code.
<https://bestia.dev/youtube/docker_rust_development.html>
In the third video, we created a simple CLI (command line interface) application to demonstrate how Rust development works in real life.
<https://bestia.dev/youtube/rust_plantuml_client.html>

Today we will go a step further to perfection.  
The Rust code can be compiled for almost every architecture and Operating system. And the CLI interface for text terminals is almost universal.  
<https://doc.rust-lang.org/rustc/platform-support.html>
What we really want is to have a Graphical user interface, but this is a problem Rust cannot help us.  
<https://www.areweguiyet.com/>
Every Operating system invented a totally incompatible library for the G.U.I. On purpose, so they can sell expensive things and have a monopoly.
There are a few universal G.U.I libraries that work on many OS, but probably not everywhere.
I think that the only true world standard for G.U.I is HTML5 with CSS3. That will probably work everywhere.
<https://www.w3.org/wiki/The_web_standards_model_-_HTML_CSS_and_JavaScript>
It is not a light solution, the HTML and CSS must be rendered by a browser. But if the whole web is created around it, that must mean it is quite good.

The G.U.I must be manipulated programmatically and unfortunately for 25 years, we were cursed to use only Javascript. That was not great at all.
Fortunately, all browser manufacturers agreed and implemented WebAssembly or WASM. The best language to be compiled for WASM is Rust. Great for us Rustaceans!
<https://webassembly.org/>
<https://www.rust-lang.org/what/wasm>

Traditional Web pages and Web Applications needed an internet connection to be online to work. For a simple utility application, we would like to use it offline.
Enter PWA (Progressive Web Application). This is a small standard that enables offline use of Web Applications and it is implemented in all modern browsers. Hooray!
<https://www.freecodecamp.org/news/what-are-progressive-web-apps/>

This demonstrative project will be fairly simple, just to show the main scaffolding needed to make a PWA using WASM and Rust.

The problem we are solving is simple: Text sort.  
I have a list of song titles and I want to sort them alphabetically.
<https://bestia.dev/guitaraoke/songs.html>
There are many, many solutions all around: online, inside text editors, as bash command,... but I need a special sorting that is not English.
Everything then falls apart ! Most of the solutions are English only. Or it is very complicated to select the desired collation (alphabetical order).

Let's get started!

First, we will create a minimal working PWA using my utility "new rust wasm pwa".  
It is also a PWA that can be used online or offline. Practice What You Preach !
<https://bestia.dev/bestia_dev_new_rust_wasm_pwa/>
We need some basic data to initialize the project like name, description, author, etc...
The name of the project will be "Sort text international Rust WASM PWA".
Finally, we need a png image of exactly 512x512 pixels for the icons.
The template is created and we can download it.

After reboot, I need to initialize my Rust development environment with a short script.
Press Ctrl-R, type "after" to search in bash history, then press tab and press enter.

We can use the fantastic Total Commander File Manager in Windows to extract the files and copy them over SSH to our Docker container.
The Secure FTP plugin is great for file operations over SSH.
To open my Directory hotlist I press Ctrl-d.  
I can work with Zip just like with any other folder. I select the content and press F5 to copy over SSH.

Now I can open VSCode, press F1 and connect to the Remote SSH Host inside my docker container.
In the VSCode terminal, I will go to the new directory and open a new VSCode window.
We have a minimal but fully functional Rust project.  
We can see all the files and folders:  
the standard License, Readme and Cargo.toml files,
the automation tasks,  
the Rust code in 3 separate modules,  
the Web server folder with the index.html, service worker, manifest and icons needed for PWA.

We can build the project simply with "cargo auto build".
After the automated task, there are instructions on how to proceed.
Open a separate bash terminal and run the simple web server only once.
Then leave that terminal to run in the background. It will constantly serve the new files as you build.
Ctrl-click on the URL to open the browser. The PWA is working.
I can now enter my name and click on the button Reload to see if the Rust code is handling the events properly.
In F12 developer tools I can debug the application.
Heureka! It works.

Now is a good time to add this project to GitHub.  
I use VSCode to initialize the repo and for the first commit.  
Then in VSCode terminal, I copy the commands from GitHub.
I use ssh-agent and ssh-add to store my SSH credentials for GitHub.

Finally, we are ready to write some Rust code. For this simple application, the html is stored inside the Rust code in a Raw string.
Raw strings are great because you can copy-paste exactly the same string without alterations.
When we change the Rust code, we compile it and then refresh the browser page. The web server works in the background all the time.
It is wise to use F12 developer's tools to debug the application.

We changed the input type text into a textarea element. The way to get and set the value is different.
The web-sys library uses only the features listed in Cargo.toml. In this case, we need to add one.
We can now read the textarea, sort it naively and set it back. Let's build and test in the browser.

This English alphabetical sort order is not good for me, I need the Slovenian because of special characters.
Rust does not have yet a stable unicode collation library.
<https://lib.rs/keywords/unicode>
<https://docs.rs/icu_provider_cldr/latest/icu_provider_cldr/>

I will use the javascript Intl Collator object to sort text for different languages.
<https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator>
<https://rustwasm.github.io/wasm-bindgen/api/js_sys/Intl/struct.Collator.html>  
Rust wasm and javascript can work very well together with web_sys and js_sys crates using wasm-bindgen.  

The web_sys_mod contains all my functions working with web_sys, js_sys, dom and html elements.
I try to isolate all javascript code and conversion in this module.
I will create a new module for sorting and use the prepared functions.
I will add an input text element for the collation.  
One line to read this value and change the call to my new sort function.
Nothing is red, so there are no errors. We can compile and test.
Control-click the link, it will open our application in the browser.
We can now sort in the "sl" Slovenian collation alphabetical order.
Heureka!! It works correctly!
The locale can be "de" for German or "hr" for Croatian, etc.
Finally, we can write in the README file all the important information and links.
I made also a screenshot of the working application so the README file is not so boring.  
I will add the link to this video when it will be ready.
To view the preview of the readme file first press Ctrl-B to close the Explorer then Ctrl-K and V to open the preview.  
We will now run our automation tasks one-by-one finishing with the publish to the web. The step-by-step instructions are very helpful.
Cargo-auto has added bash auto-completion for the cargo command and all automation tasks.
The standard tasks are in this order: build, create doc, test, commit and publish to the web.
I use ssh-agent and ssh-add to store my credentials for GitHub and web server.
Control-click the link to open the PWA from the true web server.

This is all for today.  
Thank you for watching and see you next time.  
Feel free to contact me on bestia.dev or github.

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
