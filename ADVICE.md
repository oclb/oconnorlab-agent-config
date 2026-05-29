# Luke's AI for research advice

This document is meant to give principles and big-picture advice that I think will not become stale over time.

## Try things and systematize

I previously put it this way:

>Figure out what works for you, then keep doing it

You should actively experiment in order to figure out what works for you - try things that others recommend, try giving the AI different kinds of tasks, try a new app or plugin, try voice-to-text, etc. Reflect as to whether it works for you. When you find something that works, make note of this and create some kind of system to replicate it: a skill, a rule, or most frequently just a habit. For example, here are some things that have worked for me:
- Telling the AI to ask me 3-5 questions before planning
- Creating a project-specific skill for a commonly repeated type of analysis
- Using voice-to-text and writing long, unpolished prompts
- Naming tabs in my terminal

Here are some systems that I have encoded using skills or rules:
- The AI makes a record of every analysis it performs
- The AI is instructed to ask me questions in plan mode
- All software changes go through formal review
- The AI is instructed to number its lists so I can respond more easily

Maybe you will benefit from some of these same practices, and some of them are codified in this repository, but to a large extent we all have to figure these things out as we go. Our work is not cookie-cutter, we have different preferences and capabilities, and the models themselves are evolving. You will benefit from an active-learning mindset. 


## Understand what the AI is doing

Someone on Twitter wrote something like:

> You can outsource thinking, but not understanding

This is a fundamental challenge of human-AI collaboration. With AI, we can create software without understanding how it works and produce scientific results without understanding what they mean. 

However, this challenge is not that different from that of human-human collaboration. PIs are familiar with this challenge: we must constantly make sense of work which we did not do yet for which we are responsible. When discussing new methods or results with trainees, I constantly check my own understanding by asking questions, sometimes out loud, sometimes in my head: how does the method work? What key choices were made? What could go wrong in this kind of analysis, and what sanity checks are needed? When performing analyses using the AI, you should ask these same questions. When writing software with the AI, analogous questions include: what files or modules need to change, and why? Does the relationship between modules change? What key choices are being made, or what is a key step? What could go wrong, and what test would be reassuring? Why did the AI take so long to accomplish a seemingly simple task? 

You can ask these questions to the AI directly, or just to yourself. One good time to do this is when planning; another is when a bug has been uncovered. The goal is to align the following:
- Your intent for how it should work
- Your understanding of how it works
- How it actually works

This advice applies when using the AI to perform analyses. Even though I think that AI is useful for this, this use case makes me nervous because while the AI is great at writing code, it lacks scientific judgement and integrity. Your goal should be to understand every scientifically relevant detail as though you had not used AI.  

## Structure code to help you understand it

The idiom goes:

> Form follows function

This is applicable to code, but perhaps a better slogan is "form follows understanding". Structure your code in manner that aligns with your mental model.

In software engineering, a module is just an organizational unit for code; for example, they often correspond to files. When writing code I aim to concentrate scientific logic within key modules. Then, I specify different modules at different levels of abstraction:
1. For key modules, I understand and sometimes specify their internal workings
2. For other modules, I understand and sometimes specify their external surface or interface, but internally they remain a black box
3. For the codebase as a whole, I understand and sometimes specify the relationship between modules

The key idea here is that different kinds of code correspond to different levels of abstraction. One possible heuristic is that you should understand your code at approximately the level of granularity which you would include in a Methods section or Supplementary Note. 

For example, say you task the AI to merge two genomic datasets. It spends several minutes on the task and reports success. You ask: why did it take so long? It turns out that the datasets used different genome builds, and the AI installed a liftover tool in order to merge them. This might be fine - but it is important that you know a liftover was performed, and this is the kind of detail that you would include in a manuscript.

Pay attention for signs that you are too zoomed out. One sign is that for a given change, you cannot predict what modules will be affected; this indicates that you need to improve your understanding of the structure of your codebase. Another is that you encounter bugs which affect some module recurrently; this indicates that you need to understand or modify its inner workings.

## Develop good work habits

AI poses a special challenge to people (like me) who can struggle with attention and distraction because it constantly creates 1-10 minute pauses which are opportunities for distraction. Here are some habits that I employ to remain on task; I encourage you to "figure out what works for you, and keep doing it":
- The prompt-think-prompt pattern: after sending a prompt, I actively think about things like "what's the next step?" "how would I solve this task?" "is the approach we're taking actually best?" 
- Two active sessions: I often have two agent sessions between which I alternate my attention. Other sessions may be open, but my attention remains on just two. Others report that they can maintain as many as 10 active sessions, but I cannot imagine this is the right approach for most people. 
- Make distractions less available: I stay logged out of Twitter on my work computer. Even though I could easily log in, this little friction point is sufficient to stop me from reflexively scrolling. If you find that your phone is a major distraction, for example, you could put it in a drawer instead of leaving it within hand's reach.
- Maintain a written to-do list even if you could keep it in your head: a written to-do list containing a single item is a powerful thing.
