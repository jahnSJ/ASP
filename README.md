Audio Signal Processing (ASP)
---

This project is made up of four separate Project with a similar goal.
The goal of these projects is to get data from a microphone and display the data from the microphone on a display.
To accomplish this goal I used the [STM32F29zi Microcontroller](https://www.st.com/en/evaluation-tools/nucleo-f429zi.html),
the [Adafruit 1.44" Color TFT LCD Display with MicroSD Card breakout](https://www.adafruit.com/product/2088) and the
[Adafruit I2S MEMS Microphone Breakout](https://www.adafruit.com/product/3421).
To display the audio data, I needed to differentiate between different input signals. This is done through a real-time algorithm for voice activation detection.

To see if the implementation was a success, I generated a sine wave with a 2511 Hz frequency using the [sound generator](https://onlinesound.net/tone-generator).
And the implementation was deemed a success if the sound was interpreted as sound and not silence.
So it was only necessary to differentiate between sound (voiced, unvoiced) and silence.
To get the signal I used the least mean square algorithm, which can only show the desired signal (which is a sine wave). Every time the voice activation detection notices a signal, a sine wave is displayed on the display.

The above is done with the RTIC Framework, but with RTIC I also used CMSIS for the Implementation of complex numbers and my own implementation of complex numbers in ARMv7M-E ISA, which is compatible with the STM32F29zi Microcontroller.
To use the assembly code I wrote an implementation of it in C and to use it in rust I wrote a wrapper.

License
---
This project is double licensed under [MIT](https://opensource.org/license/MIT) and [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt), because the code used in this project uses library that use these licenses.
