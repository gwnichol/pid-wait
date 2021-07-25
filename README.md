# Pid-wait

Wait for a process to complete. This tool is usefull to trigger some other command after process completion.

---

Have a alias to easily notify on command completion?

Do you hate when you forget to use it when invoking a long build job?

This tool can help you!

---

## Example usage

Search for the build job and use the hypothetical "notify":
```
pid-wait -s make && notify "The build is done"
```

If you already know the pid you want to connect to, use this:
```
pid-wait -p 1 && notify "Init exited? How?"
```

## License

Copyright 2021 Grant Nichol

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
[https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0)> or the MIT license
<LICENSE-MIT or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)>, at your
option. Files in the project may not be
copied, modified, or distributed except according to those terms.