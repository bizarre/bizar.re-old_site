---
title: 👨🏾‍💻🏃🏽💨
subtitle:
---
anthony hooked up the frontend to the backend pr quickly. a user was now able to click the `run` button and their browser would send their code solution to our backend for *processing*. 

*processing* is what i had to figure out. 

again, we worked on tavern as if it was a product we wanted to take to market. this meant that we **had** to make sure that we did not allow for the unsafe execution of malicious code that could leak site data or do something harmful. if i had more time, i would've liked to have created a pl-agnostic docker solution where we could have specific images and just pipe user code into one of these safe containers and capture output. unfortunately, at this point in time, we had maybe ~8 hours left, i was on my fucking ninth redbull sweating ass and hell because we still weren't really close to demo level ready. 

each language had its own implementation for execution, but ill go over js cuz its pr straight foward. bc docker was out of the question due to time constraints, i had to scour the internet real mf quick to find something that would give us similar security assurance while being quick to implement. came across [vm2](https://github.com/patriksimek/vm2), an open-source node lib that lets u run untrusted code in a *vm*. implementation was hella easy, if not pretty hacky and fuck-shit-level-exploitable:

```javascript
const vm = new VM({
  timeout,
  console: 'redirect',
  sandbox: {
    console: {
      log: function (msg) {
        logs.push(msg)
      },
    },
  },
})

const input = JSON.parse(testCase.input)
const output = JSON.parse(testCase.output)

let toExecute = entrypoint
Object.keys(input).forEach((key) => {
  const value = input[key]
  toExecute = toExecute.replace(
    `$${parseInt(key) + 1}`,
    `${JSON.stringify(value)}`
  )
})

let result = null
let stacktrace = null
try {
  result = vm.run(`${body}${toExecute}`)
} catch (e) {
  stacktrace = e.stack
}
```

basically piped output and pushed it to a list so could send back to users and they could debug their code like professionals with a ton of print statements 😎. also wrapped the execution in a try catch so we could grab the stacktrace (if any) and send that to user too. tiny problem w/ this tho is stacktrace contains refs to the vm2 code so tracing it is a little convoluted but it does still contain valuable information for fixing any kind of syntax error, etc.

given how fast the code runner was implemented, how it does not sacrifice the security of the project, how it intuitively hooks into the data provided by the templating system and supports multiple languages, i am very happy with the implementation. once i was able to respond to clients `run-code` request with all this information, it didn't take long for anthony to tie it up and polish it out. the end result is a very clean looking code editor page.