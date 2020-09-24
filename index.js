#!/usr/bin/env node

let [ _N, _P ,time, err ] = process.argv
time = parseInt(time)
time = isNaN(time) ? 7 : time
err = parseInt(err)
err = isNaN(err) ? 1 : time
console.log(`Exiting in ${time}s with error #${err}`)

setTimeout(() => process.exit(err), time * 1000)
