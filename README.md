# Translata = Translator Assistant via Web

Translata是一个通过Web上的翻译网站进行翻译的辅助工具。  
Translata is a tool to translate via a translator on web.

因为工作的关系，我经常会学习一些英文的技术文档。在学习过程中，为了加深印象，我通常会将文档翻译成中文。  
I need to learn some English technical documents frequently because of my job. To make myself understand those documents completely, I usually translate documents into Chinese when learning.

但是，对于我的用例而言，现有的翻译工具的使用方法都比较繁琐。我需要先从编辑器中复制需要翻译的文本，然后粘贴到工具输入框，等待翻译结果出现，再选定并复制，然后粘贴回编辑器。  
However, for my use case, the use of existing translation tools is kind of cumbersome. I need to copy the text to be translated from the editor, then paste that into the input box of the tool, wait for the translation result to appear, then select and copy, and then paste back to my editor.

另外，这份工作虽然可以使用云端API来完成，但API不是免费的。  
In addition, although we could use the cloud API to achieve our goal, it is not free.

因此，我实现了这个工具。这样，在翻译过程中，我不需要离开我的编辑器。只需要先复制我需要翻译的文本，按下快捷键（Ctrl-T），移动光标到目标位置，粘贴翻译后的结果并修改即可。换句话说，这个工具代替我在浏览器中做了一次粘贴和复制。  
Therefore, I implemented this tool. Now, I don't need to leave my editor when translating. I just need to copy the text to be translated, press the shortcut key (CTRL-T), move the cursor to the target position, paste the translated text and do next modification. In other words, this tool pasted text and copied result in the browser for me.

需要注意的是，这个工具目前的实现还非常简陋，而且很可能由于网站（目前使用Google Translator）改版而停止工作。如果这个工具对你有用，请告诉我。  
It should be noted that this tool is still very simple, and it is very likely to stop working because of the website (currently using Google Translator) changes. If this tool is useful to you, please let me know.