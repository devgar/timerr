#!/usr/bin/env node

const [_, _ ,time, err] = process.argv
let time = parseInt(time)
time = time || 7
let err = parseInt(err)
err = err || 1
console.log(`Exiting in ${time}s with error #${err}`)

setTimeout(() => process.exit(err), time * 1000)
