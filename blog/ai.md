# AI is Unsustainable

There's been an insane spike in AI usage over the past years. It's to the point that I don't think I even need to explain that sentence - you probably know it just as well as I do.

Although, if you do want some numbers:
- ChatGPT's number of weekly users [doubled from 100 million to 200 million users](https://www.theverge.com/2024/8/29/24231685/openai-chatgpt-200-million-weekly-users) between November of 2023 and August of 2024 (less than a year!).
- ChatGPT released in November of 2022. In the *two years* since then, there have been at least 3 more major company-backed LLMs released (Google's [Gemini](https://gemini.google.com), Anthropic's [Claude](https://claude.ai), and Facebook's [Llama](https://www.llama.com/)).
- By March of 2023, according to Pew Research, nearly [60% of US adults](https://www.pewresearch.org/short-reads/2023/05/24/a-majority-of-americans-have-heard-of-chatgpt-but-few-have-tried-it-themselves/) had heard of ChatGPT (and let's be honest - getting 60% of US adults to know about *anything* is impressive!).

I've even had a teacher *encourage AI usage* on assignments.

> **Note:**
>
> Technically, using "AI" here is a bit of a misnomer. Artificial intelligence is an *incredibly* broad field, and I'm only really talking about one specific aspect of it: Commercial LLMs, the **L**arge **L**anguage **M**odels (like ChatGPT and Gemini) being developed commercially by companies. I'm still going to say AI in this post so it's easy to read, but I wanted to leave this disclaimer to remain accurate.



# The Problem

While it's cool to see a product spreading so rapidly, it's unfortunately a very problematic technology. There are *tons* if problems and questions with AI. Many of these problems aren't ones I'm going to address in this post, because they can get rather philosophical and lengthy - I'm not, for example, going to discuss if this technology is truly intelligent.

However, there are three objective problems with AI that you should be aware of:

1. AI is damaging to the environment, using absurd amounts of water and power. Its power usage is also causing increases in carbon emissions.
2. Companies are overselling the future of AI. It will not grow to meet their claims.
3. AI is not financially sustainable, nor profitable. It is draining immense sums of money from our economy.



# I Don't Care

These problems seem large and abstract, and I can completely understand someone just not caring. It's easy to write this off as something that doesn't concern us, or something that's too big to change. I thought for a while that this tech would just breeze over and we'd move on after a bit... but, it hasn't. Instead of breezing over, AI has only become *more* relevant to me and to you. Let me explain how:

- AI is relevant because companies are forcing it onto us. iOS, macOS, Android, and Windows are now all introducing (or planning to introduce) AI technology directly onto our smartphones and computers. This technology - which wasn't even relevant until the release of ChatGPT 2 years ago - is now being shoved into everything from [web searches](https://blog.google/products/search/generative-ai-google-search-may-2024/) to [app notifications](https://support.apple.com/guide/iphone/summarize-notifications-reduce-interruptions-iph1fbe7d2b9/ios).
- AI is relevant because it's affecting the planet we live on and the economy we live in. As I'll show later in this post, AI is devouring water, electricity, and money at a faster rate than any technology we've seen before. This can worsen droughts in areas low on water and adds strain to the economy we all have to deal with.
- AI is relevant because remaining ignorant on the topic allows companies to take advantage of us. For as long as people remain ignorant on the topic, companies will manipulate those people by pushing more AI products to them. Doing so maintains hype, attracts potential customers, and draws in investors - all of which boosts companies' precious profits. Just being more aware of AI and its downsides makes you more resilient to this manipulation.

I'm writing this post because AI is relevant to you. You deserve to know its consequences, and what you can do to address them.



# Compute Power Needed for AI

I want to start off with the first fact that companies never mention - AI is *super* resource-intensive to run. Sure - we've all had apps that take forever to load; computers taking ages to boot; the videogame that makes your laptop's fans run so hard, it sounds like the laptop's going to take off like a hovercraft - but this is on a different level. AI requires entire *datacenters* of compute power to run. We have buildings, today, that do nothing except process the requests of AI users across the world.

It may sound like I'm being vague about exactly how much compute power AI needs, and you'd be right. AI companies hide figures like this; we *don't get* to know how many datacenters AI companies use [or even where those datacenters are](https://community.openai.com/t/location-of-openai-servers/559880).

However, there are hints as to how much compute power they need. OpenAI seriously proposed building [multiple 5-gigawatt datacenters](https://arstechnica.com/tech-policy/2024/09/openai-asked-us-to-approve-energy-guzzling-5gw-data-centers-report-says/) across the US (5 gigawatts being roughly equivalent to 5 nuclear reactors' worth of energy). They've also stated in their blog that they planned on using [thousands to tens of thousands](https://openai.com/index/openai-and-microsoft/) of datacenter computers owned by Microsoft for their programs. That blog post is from all the way back in 2016 - six years before ChatGPT was released - and OpenAI's computing needs have absolutely risen dramatically since then. In a much newer blogpost announcing the release of GPT-4, OpenAI even mentions [building a literal supercomputer](https://openai.com/index/gpt-4-research/) to run the model.

<details>
<summary>Side Experiment: Just How Much Compute Power is Needed for LLMs?</summary>
<div>

You can actually run Meta's [Llama](https://llama.com) LLM locally on your machine with [ollama](https://ollama.com). I have a pretty powerful machine, and was curious how an LLM would perform on it.

> **Warning:**
>
> This section ended up being super long. Like, it's long enough to warrant its own blog post. You can skip to the bottom for the conclusion.

For reference, my PC's specs are:

- AMD Ryzen 9 5900X CPU (12 cores, 24 threads, overclocked to 4.9ghz)
- 32gb of RAM
- AMD Radeon RX 6800 GPU (16gb VRAM, 60 compute units, 1815mhz)
- Running Arch Linux 6.11

This is a pretty chunky build - I easily play any game, usually at high (sometimes max) graphics. I can run Cyberpunk 2077 raytraced at roughly 70fps. I can also crack passwords in the `rockyou.txt` list in under 30s. [PCGameBenchmark](https://www.pcgamebenchmark.com/ratemypc?platform=linux&cpu=amd-ryzen-9-5900x&memory=32gb&gpu=amd-radeon-rx-6800) rates my setup at 98%; more powerful machines can be built, but my build approaches top-of-the-line for common consumers. Just keep this in mind when I discuss the performance of the LLMs below.

Back to LLMs - an LLM's size can more-or-less be compared by the number of parameters it has. I'm not going to go over what exactly a parameter is, but they kind of make up the "brains" of an LLM. More parameters allow an LLM to generate better output (assuming it's trained well), but they will also require more compute power to run. GPT-3 has [175 billion parameters](https://en.wikipedia.org/wiki/GPT-3). It's not publicly known how many parameters GPT-4 has, but [it's rumoured to be over 1 trillion](https://en.wikipedia.org/wiki/GPT-4#Background).

The largest Llama model I found from a quick search seemed to be the [Llama 3.1 405b](https://ollama.com/library/llama3.1:405b) model. The 405b means it has 405 billion parameters, which is even larger than GPT-3. Unfortunately, this model requires *over 200gb* of free space to install. That would take forever to download, and I'd have to actually clean my files to make room for it... which, like, ew.

So I went with the next biggest one: The [Llama 3.1 70b](https://ollama.com/library/llama3.1:70b) model (again, 70b meaning 70 billion parameters). This model needed 40gb of free space. The following is my experience running it.

First off, the model took roughly an hour to download because it's 40gb. In the meantime I played around with the smaller [Llama 3.2 model](https://ollama.com/library/llama3.2), which clocks in at 3 billion parameters and only needs 2gb of storage space. Much smaller.

I actually liked Llama 3.2; it was small, generated answers quickly, and I could use it right from my terminal. If I didn't have to worry about AI hallucinations, I'd probably find myself using it since it's so fast. I asked it a few basic questions about history and got it to write a few basic programs. It knew major world events and could write some Python; though it really tripped up when I asked it to write Rust (yay for my job security?). Considering how much smaller it was than GPT-3, I wasn't really expecting much, and was honestly kind of surprised at how well it did. I assumed I'd notice a difference from ChatGPT immediately, but it seemed fine in the few minutes I used it (granted, I've not used ChatGPT very extensively either).

> **Note:**
>
> For non-programmers, Rust and Python are both programming languages; I just asked Llama to write simple programs in both.

Performance-wise, I was already starting to get worried, though - queries were taking up a noticeable amount of CPU:

<img src="data:img/png;base64,#!INCLUDE_BASE64(../assets/ollama-3.2-compressed.jpg)" />

> I know there's a lot going on in this screenshot. The left half is just output from the LLM. The right half the important part - it's measuring how much of my hardware is being used. The topmost bar graph is CPU usage, the middle (purple) one is RAM/memory usage, and the bottom one is network usage. The important ones are the CPU usage and RAM usage.

The three spikes I drew arrows to were three of the queries I ran. The spikes on their own weren't concerning; but the fact that there's already spikes doesn't bode well for trying to run the 70b model.

The first spike was from the prompt "generate a 100 word story". The second was "generate a 2000 word story", which understandably took longer and resulted in a longer spike. The third prompt was "write a hello world program in Rust using the egui library", which resulted in a shorter but noticeably higher spike.

AI is supposed to be GPU-intensive, though, not CPU-intensive, so my next step was to monitor GPU usage while running some prompts. I started a new session and ran the exact same three prompts, measuring GPU usage with `nvtop`:

<img src="data:img/png;base64,#!INCLUDE_BASE64(../assets/ollama-3.2-gpu-compressed.jpg)" />

The orange line is GPU memory usage, and the green line is GPU compute usage. The very first spike in this image is just from starting Llama, and the other three are the same prompts as before. I did switch back to my IDE in the middle of the 2000-word-story prompt, which may have affected GPU usage slightly (my code editor uses GPU-accelerated rendering); otherwise, this data should be pretty good.

The model was already using 22% of my GPU memory, which was again a little concerning since this model is small. The prompts maxed out GPU usage, but I think this is expected - it's just using its resources well.

After experimenting with 3.2 and writing this portion of the blog, the download for 3.1 70b model was almost done. This model has over 22x as many parameters, so I was worried if it'd run at all after seeing 3.2's memory usage.

I tried to run it for the first time, and... it immediately errored! Turns out this model needs a whopping 26gb of memory to run. I have 32gb of memory, but also had a lot of apps open. I ended up having to close every single app that was open to have enough memory for the model - no web browser, no Discord, not even the code editor I was writing this post in (I had to take notes on paper like a plebeian).

Amazingly, things only got worse from there.

The model took one and a half minutes to even start. Performance was looking like this:

<img src="data:img/png;base64,#!INCLUDE_BASE64(../assets/ollama-3.1-startup-compressed.jpg)" />

(I got smart and measured GPU usage at the same time as system usage. They're both the same programs as before - the top half is the CPU/memory/network usage, the bottom half is the GPU usage.)

After a very painful minute and a half, it finally opened. I started with a basic "who are you" to let the AI introduce itself, and was stunned by how slow it was.

The 3.2 model had generated tokens faster than ChatGPT on the web. It gave quick answers and felt responsive.

This one was generating one token every 1-2 seconds.

> **Note:**
>
> If you don't know what tokens are, I'll explain later in the blog. Just pretend a token is an English word for now.

I tried to get a screenshot of the resource usage while the AI was generating a response. This then caused the LLM to crash, and I had to restart the whole thing, including another minute-and-a-half startup time. *ugh*.

I didn't take any more screenshots until after I shut down the model, to avoid any more crashes. I ran 3 more prompts, and timed the responses for each one:

- "write a 50-word story": 1 minute, 10 seconds
- "write a python program that adds two numbers and prints the result": 3 minutes, 3 seconds (to give some credit to the AI, it decided to write two programs here. The second was identical to the first but showed off Python's f-strings feature).
- "write a hello world in rust using egui": 6 minutes, 35 seconds (the program it wrote also didn't work in the end because the AI invented a library called "epi").

> If you're wondering what the outputs for these prompts were, I got a little too eager when shutting down the AI so I could open all my apps again, and accidentally closed the AI chat window. So the output's been lost to history...

After waiting an amazing 6 minutes for the AI to write a basic Rust program, I closed everything and grabbed one last performance screenshot:

<img src="data:img/png;base64,#!INCLUDE_BASE64(../assets/ollama-3.1-shutdown-compressed.jpg)" />

The GPU has a lot of spikes, but overall usage was pretty low. In comparison, the 3.2 model consistently used at least 80% of the GPU when it was processing prompts. I think what happened is my GPU didn't have enough VRAM to load the entire AI at once, so it was actually having to load parts of it at a time while processing the prompt. Either that, or it was running part of the model on my CPU instead of my GPU, because my CPU was pretty consistently pushed to the max while running prompts.

Keeping this performance in mind, let's compare to OpenAI. They're running an AI with *several* times as many parameters as the one I just attempted. They're also doing it concurrently to generate responses for tons of users at the same time, since they have 200 million weekly users.

I think it's safe to say OpenAI is using *insane* amounts of compute power, and saying they run ChatGPT on supercomputers isn't an exaggeration.

</div>
</details>



# Environmental Impact

Since the tech needs such an incredible amount of compute power, it's going to have an environmental impact. The companies developing LLMs don't release statistics on the environmental impact of those LLMs (of course they don't - it's bad PR!), so we can't find exact details on the environmental impact of LLMs. That being said, there is a way for us to get a rough feeling for the environmental impact of AI.

As stated before, AI programs run in massive datacenters. Although we don't have figures for the environmental impact of AI, we do have figures for the environmental impact of datacenters. Companies like Google and Microsoft publish yearly reports on their environmental impact, which we can use to analyze the impact of their datacenters.



## Water Usage

Computers, obviously, get quite hot when pushed to their limits. For smaller computers like laptops, a couple of fans is generally enough to solve this - the fans can pull cold air in and push hot air out, preventing the system from overheating.

Datacenters, however, are an entirely different beast. They have so many computers and generate so much heat that fans just aren't enough to cool them. Instead, they rely on freshwater for cooling. Unfortunately, datacenters use a *lot* of water to do this - according to a [2023 study from the University of California](https://arxiv.org/pdf/2304.03271), datacenters were responsible for the consumption of somewhere between 4.2 and 6.6 billion cubic meters of freshwater for cooling.

Numbers that big don't really make sense without context, so the study also gives us this comparison: Datacenters are using just as much water as half of the entire United Kingdom, every year.

<details>
<summary>Comparing Water Usage to the Pandemic</summary>
<div>

> **Warning:**
>
> I made this part its own section because it's somewhat speculative. It doesn't have the same degree of certainty as the rest of this post.

During lockdown for COVID-19, the usage of internet services skyrocketed between [40% and 100%](https://pmc.ncbi.nlm.nih.gov/articles/PMC7280123/) (depending on the service). Because of this increased traffic, we'd expect to see increased datacenter usage (and therefore environmental impact) between 2020 and 2022. ChatGPT released in 2022, and LLMs have really kicked off since then.

With this context, we should be able to see if AI is having a real, measurable impact on water usage by comparing the water usage of datacenters before, during, and after the pandemic. So, then - what is that water usage?

<img src="data:img/png;base64,#!INCLUDE_BASE64(../assets/microsoft-water-usage.png)" />

> **Table Explanation:**
>
> This table comes from Microsoft's [2024 environmental report fact sheet](https://query.prod.cms.rt.microsoft.com/cms/api/am/binary/RW1lmju). I chose Microsoft in particular because OpenAI uses Microsoft's Azure compute services, so ChatGPT has a direct effect on Microsoft's datacenters' environmental impacts.
>
> The columns are "fiscal years", which run from the July 1st of the previous year to June 30th of the current year. So, for example, FY23 ran from July 1st, 2022 to June 30th, 2023.

As expected, water usage climbed during fiscal years 21 and 22, during the pandemic. But, interestingly, water usage continued to climb after the pandemic ended - by more than 22%!

This seems to me to be the direct result of AI. Internet usage skyrocketed *so much* during lockdown because of how many things we transitioned to being virtual. School, meetings, and even just buying groceries *all* went digital, the vast majority of which went back to in-person after lockdown.

It also doesn't seem likely that there's really another explanation for such a drastic increase in water usage. ChatGPT became the [fastest growing platform ever](https://www.reuters.com/technology/chatgpt-sets-record-fastest-growing-user-base-analyst-note-2023-02-01/) within months of its launch - Microsoft hasn't had anything else grow even nearly as explosively since lockdown ended.

</div>
</details>


## Energy Usage

Let's switch gears and talk about electricity. Computers use power to run. At smaller scales like laptops and phones, power usage is pretty low, and we don't really think about it. However, at the scale of an entire datacenter, power usage becomes *immense*:

- In 2023, datacenters made up somewhere between [1%-2% of the *entire world's* electricity usage](https://arxiv.org/pdf/2304.03271).
- A single datacenter can use [just as much power as 50,000 homes](https://thereader.mitpress.mit.edu/the-staggering-ecological-impacts-of-computation-and-the-cloud/).
- According to Google's [2024 environmental report](https://sustainability.google/reports/google-2024-environmental-report/), their carbon emissions have risen by 48% since 2019. There also appears to be a noticeable spike in emissions in 2023, the year they trained and released Gemini:

<img src="data:img/png;base64,#!INCLUDE_BASE64(../assets/google-carbon-emissions.png)" />

Now, obviously, there's a lot of things that contribute to our environmental impact besides AI, and even besides datacenters. But any time a company brags about new AI features, or new statistics come out about the number of people using LLMs, keep these numbers in mind. Those new features and users are having a *real* impact on our power and water usage - an impact companies don't seem keen on talking about.



# Fleeting Training Data

The next problem with AI that I'll discuss today is its disappearing training data. I'm going to give a brief overview of how LLMs work as some context for this issue.

The GPT in ChatGPT stands for "Generative Pretrained Transformer" - it's a label for the category of AI that LLMs fall under, and explains how they work.

GPTs are "generative" because they - major spoilers here - generate new content. When you give an LLM a series of words, the LLM will predict the next word that should follow it. A response is generated by predicting the next word, adding it to the response, then repeating that over and over until the response is complete. LLMs predict the next word based on a kind of word database they have, which stores related words closer together.

"Pretrained" refers to the fact that GPTs/LLMs receive training before they're used. That training basically updates the word database the LLM uses and makes it more accurate.

"Transformer" refers to the math that makes GPTs work; but I'm not going to get into that here.

> **Note:**
>
> Technically, LLMs work with *tokens*, not words. Tokens are how LLMs break down English, and may be words or pieces of words.
>
> Here's some examples of tokens for ChatGPT, with each token separated by a "/":
> - "Hi, my name is Bob" is broken down into the tokens "Hi/ my/ name/ is/ Bob"
> - "the orange cat meows" is broken down into the tokens "the/ orange/ cat/ me/ows" (notice that "meows" gets split into 2 tokens)
>
> You can test other phrases yourself here: https://platform.openai.com/tokenizer.
>
> For simplicity, I'm going to keep saying "words", not "tokens", but tokens would be more accurate.

The important thing to realise here is that an LLM's performance depends largely on its training. The training it receives creates the database that it uses to generate content. Too little good data or lots of bad data will make a bad LLM; lots of very good data will make a great LLM. To keep improving the LLM, you'll need even more high-quality training data.

> **Note:**
>
> I'm oversimplifying *greatly* here for the sake of explaining this problem. There's a lot more factors that go into GPTs.
>
> If you want a more in-depth explanation on how GPTs work, 3Blue1Brown has a great series on YouTube about it:
>
> <iframe loading="lazy" width="560" height="315" src="https://www.youtube-nocookie.com/embed/wjZofJX0v4M" title="YouTube video player" frameborder="0"></iframe>

Since AI needs tons of high-quality training data, that data comes from the internet. There's no other source that can provide such vast amounts of content for training data. However, not all internet content is going to make good training data. A satirical article from The Onion or a piece of propaganda isn't going to be good for the AI, because then it might generate content with misinformation. Ironically, [AI-generated content *also* isn't high-quality enough to train AI](https://www.scientificamerican.com/article/ai-generated-data-can-poison-future-ai-models/), because any errors it makes will get reinforced in AI that trains on it.

This essentially means that training data has to be curated and filtered down to a single set of "good data" that will improve the AI. OpenAI talks about this in their [paper about GPT-3](https://arxiv.org/pdf/2005.14165#subsection.2.2) - they say they took a massive collection of 45tb of text and filtered it down to 570gb, which was then added to the dataset they trained GPT-3 on. That's a lot of cut data - they only kept roughly 1.3% of it!

The problem (for AI) is that people have started adding restrictions to their content that specifically prevents using their content as training data for AI. According to a [recent study](https://www.dataprovenance.org/Consent_in_Crisis.pdf), many of the sites that have historically been goldmines for training data now have as much as 45% of their content restricted this way. The same study states that these restrictions have increased more than 500% compared to last year. In short, the most-used sources for AI training data are becoming less and less available over time.

So when [Sam Altman](https://www.tomsguide.com/ai/chatgpt/sam-altman-claims-agi-is-coming-in-2025-and-machines-will-be-able-to-think-like-humans-when-it-happens) promises we'll have "artificial general intelligence" that thinks like humans in the next few years, it's nothing but marketing. It's a shiny prediction that makes OpenAI sound amazing, which gets the company more investors and allows them to stay afloat.

The reality is that long before we have AI as intelligent as humans, or build a Skynet that tries to eradicate us all, we're going to run out of training data to keep improving AI. It's simply going to stagnate.

Now, there is a potential solution to this problem: AI companies could simply pay people to write new content, and use that as training data. Unfortunately, that's unlikely to happen, because...



# Financial Cost

To me, this is one of the stupid parts of this whole situation: AI, as a product, *is not profitable*.

This might seem counterintuitive, since most AI products are free or have an optional subscription. However, as discussed at the start of this post, AI needs a *lot* of compute power and energy to run, and someone has to pay for that. Companies hide these costs from end users when the make AI products free. For now, AI companies are able to stay afloat anyways, because private investors are pouring [literal billions of dollars](https://openai.com/index/scale-the-benefits-of-ai/) into the tech.

Companies like OpenAI and Anthropic don't publish their financial details publicly, which makes it hard to know exactly what their income and expenses are. However, they do share financial documents with investors when raising funding. As it turns out, the New York Times managed to get their hands on one of OpenAI's investor documents, and they published their analysis of it in an article. According to [that article](https://www.nytimes.com/2024/09/27/technology/openai-chatgpt-investors-funding.html), OpenAI is set to make $3.7 billion this year - but the company will still face a net loss of $5 billion. Furthermore, the Times says some expenses are missing from that $5 billion figure, so their losses could be even larger.

This isn't the first year OpenAI has lost money, either. In 2022, while developing ChatGPT, they [reportedly lost $548 million](https://web.archive.org/web/20230619191257/https://www.theinformation.com/articles/openais-losses-doubled-to-540-million-as-it-developed-chatgpt).

Let's put this in a timeline, along with some other data to give more context about OpenAI's growth and finances.

<table>
    <tbody>
    <tr>
        <th>November 2022</th>
        <td>OpenAI releases ChatGPT</td>
    </tr>
    <tr>
        <th>December 2022</th>
        <td>OpenAI loses a reported $548 million</td>
    </tr>
    <tr>
        <th>January 2023</th>
        <td>
            ChatGPT reaches 100 million monthly users,
            becoming the <a href="https://www.reuters.com/technology/chatgpt-sets-record-fastest-growing-user-base-analyst-note-2023-02-01/">fastest growing platform ever</a>
        </td>
    </tr>
    <tr>
        <th>February 2023</th>
        <td>
            OpenAI releases <a href="https://openai.com/index/chatgpt-plus/">ChatGPT Plus</a>,
            a subscription for ChatGPT and source of revenue
        </td>
    </tr>
    <tr>
        <th>November 2023</th>
        <td>OpenAI reaches 100 million weekly users</td>
    </tr>
    <tr>
        <th>August 2024</th>
        <td>
            OpenAI reaches 200 million weekly users,
            doubling its weekly user count from 2023
        </td>
    </tr>
    <tr>
        <th>December 2024</th>
        <td>OpenAI is set to lose $5 billion</td>
    </tr>
    </tbody>
</table>

After releasing a subscription plan, becoming the fastest growing platform ever, then doubling their weekly userbase, OpenAI's yearly losses have grown tenfold. AI is, quite simply, *not profitable*.


## "Temporary Investments"

I've seen some people claim these losses in AI are just temporary - a small investment now for a tech that will blow up in the future. They compare to companies like Amazon, which [lost money for years](https://www.bbc.com/culture/article/20240628-a-36-year-old-jeff-bezos-talks-about-losing-money) after being founded.

To be profitable, AI companies will have to start charging for access to their services. There's no other source of income for them - [ads don't even make enough to fund YouTube](https://www.wsj.com/articles/viewers-dont-add-up-to-profit-for-youtube-1424897967), and there's no other product for them to sell.

This means, to be profitable, ChatGPT would have to go from a free service with an optional [$20/month subscription](https://openai.com/index/chatgpt-plus/) to a product with a required $20/month subscription (likely even more expensive, considering OpenAI's losses). Would you really be willing (or even able) to pay $240/year for a product that still hallucinates and is unlikely to keep improving? Do you think the majority of current ChatGPT users would pay that price?

Amazon always had the potential to make money because they're free to use and every user results in profit for them. For AI, every new user is an increase in server costs. This isn't a crazy new product that will become the next iPhone - it's a fundamentally unsustainable technology that cannot make profits unless users pay every penny of its insane costs upfront.



# What can I do?

AI is currently funded by private investments and some of the wealthiest corporations in existence. It's not some government policy that can be voted against or protested; it's being created through raw economic power.

This economic power still has a weakness, though - it feeds on public opinion, and more specifically, misinformation. Here's some examples of that:
- As mentioned before, OpenAI doesn't publicly release financial statements. Because there's few public, concrete numbers for its finances, all of us in the general public really have to rely on word-of-mouth and gut instinct about how they're doing financially. This allows them to make it *seem* like they're making money, because they always publicly mention the billions they raise in investments without ever discussing their even larger losses.
- In some cases, companies call software "AI" that is actually a completely unrelated technology. The [Rabbit R1](https://www.rabbit.tech/), for example, [raised tens of millions in funding](https://www.rabbit.tech/newsroom/rabbit-raises-additional-10m) for a cheap, handheld AI product. It was later revealed to actually rely on [simple scripts](https://youtu.be/zLvFc_24vSM?t=235) from a well-known project called [Playwright](https://playwright.dev/), and really provided no AI services at all besides ChatGPT queries.
- Finally, companies profit by overstating what AI will do. There was Sam Altman claiming his company would invent AGI, but even other companies like Apple have oversold their AI products. [Apple Intelligence](https://www.apple.com/apple-intelligence/) has seen tons of [delays](https://9to5mac.com/2024/10/23/why-the-apple-intelligence-delays-what-about-the-new-siri-craig-federighi-explains-video/), and has [mixed results according to MKBHD](https://www.youtube.com/watch?v=haDjmBT9tu4) (MKBHD actually *specifically* calls out this exact point!).

So, then - what can *you* do about these issues with AI? Well, surprisingly, it's quite simple: **talk about what you know**. Knowledge is power, so spread what you've read here, and any additional details you learn in your own research. Since AI is running largely on mislead public opinion, spreading the truth is one of the simplest ways to cut through its nonsense. This is actually the exact reason I wrote this blog post to begin with.

Another way you can help fight AI by **not paying for AI products**. Every single time a person pays for a ChatGPT subscription, purchases a Rabbit R1, or pays for some other kind of AI service, it makes the tech look slightly more profitable. That promise of profits then attracts investors, who can pay to offset the losses from running AI products, which keeps this whole gross cycle continuing.

Talking about what you know will slowly change the opinion of the general public. And as public opinion shifts away from AI hype, investors will start to follow. Making AI less profitable will stack on top of this and continue to discourage investors. And once investors back out, companies will have to finally face the real costs of AI head-on; the financial incentive to constantly push AI products will be completely destroyed. Then, at long last, all of this AI nonsense can finally breeze over.
