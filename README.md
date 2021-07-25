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