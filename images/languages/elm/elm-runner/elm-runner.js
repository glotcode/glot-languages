#!/usr/bin/env node

const fs = require("fs");
const process = require("process");
const { Script } = require("vm");
const { JSDOM } = require("jsdom");

const elmFile = process.argv[2];
const elmJs = fs.readFileSync(elmFile).toString();

const script = new Script(`
    ${elmJs};
    Elm.Main.init({
        node: document.getElementById('elm')
    })
`);

const dom = new JSDOM(`<html><body><div id="elm"></div></body></html>`, {
    runScripts: 'dangerously'
});

const ctx = dom.getInternalVMContext();
script.runInContext(ctx);

setTimeout(function() {
    const content = dom.window.document.body.innerHTML;
    process.stdout.write(content);
}, 0);
