#!/usr/bin/env node
[_, _ ,time, err] = process.argv
time = parseInt(time)
time = time || 7
err = parseInt(err)
err = err || 1
console.log(`Exiting in ${time}s with error #${err}`)

setTimeout(() => process.exit(1), 7 * 1000)
