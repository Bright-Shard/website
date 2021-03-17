---
layout: page
title: Shell Striker
sidebar:
  nav: projects
---
# ***THIS PROGRAM IS MADE FOR EDUCATIONAL PURPOSES ONLY. DO. NOT. USE. THIS. PROGRAM. ILLEGALLY.***
## Why are you making hacking stuff if hacking is illegal?
The truth is, **hacking is not (always) illegal**. What is illegal is cybercriminals, the horrible people abusing hacking to take control of your accounts. Hacking is legal as long as you have permission from the machine's owner to hack it, or if you own the machine yourself. #HackingIsNotACrime

## Are you *sure* hacking isn't illegal?
Yes. If you want more proof, or want to learn hacking, check out [TryHackMe](https://tryhackme.com)! The website allows *legal* hacking of their machines, once you make an account and connect with OpenVPN.

## What is Shell Striker for?
In many CTFs, HackTheBox machines, or TryHackMe rooms, there are vulnerabilities in outdated software. The purpose of Shell Striker is to automate vulnerability scanning.

## Um, isn't there already [NMap/Nikto/Nessus/Masscan/etc.]?
Yes, and those programs are awesome! Scanners such as those is what Shell Striker relies on. It will use a scanner to poke at a machine and then automatically search for exploits in vulnerable software versions. Shell Striker will rely on multiple scanners and is linked to Metasploit(And the Exploit DB), so you can also exploit machines directly from Shell Striker, instead of opening a new app/terminal tab.

## Can't I just look for exploits myself?
You can, but it will take longer. In competitive events like HTB bloods or CTFs, you need automation to get the edge on competitors. While they're googling exploits, you will already have the results in Shell Striker.

# Can I run Shell Striker in [insert OS name here]?
Theoretically, you can run Shell Striker on any platform, because I use [Beeware](https://beeware.org) in my apps. Beeware lets you write an app once, in Python, and then deploy it to all operating systems - including mobile! The only issue is, Shell Striker requires scanners like NMap to run. It's not easy to put a binary in my app and then just have it *work*; new versions could come out, or it might not support an OS I want to put Shell Striker on. I've been thinking of having a server, based in Linux, that Windows, MacOS, and mobile Shell Striker apps could connect to for it to run; however, I'm not sure how easy or well that would work. So, the first release will probably be linux-only.

# There's not a release for Shell Striker yet. I will develop it after I finish Hypertasker.
