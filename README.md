# Audio Serializer

An attempt to store large audio files in smaller formats without losing quality

## Results
Turns out, storing an audio buffer in a JSON makes it larger, as it has to store an array containing an insane amount of values

Sampling the amplitude of the audio signal at every given time interval (in µs) results in a massive list of numbers, as the typical sample rate (44.1kHz) will sample the signal at every $\frac{1}{44100} = 22.67\mu s$

## Testing
The same MP3 file resulted in a much, much larger JSON
| Format | Duration | Size | Array Size |
| - | - | - | - |
| MP3  | 3.5min   | 4.2MB | _ |
| JSON | _        | 79.2MB | ~18million ints |

That's a ~20x increase in size!!

## What is This?
A learning experience, I suppose. Watch [Reducible](https://www.youtube.com/@Reducible/videos)'s videos about signal processing. Amazing stuff

## Thanks to
- [rayyanhunerkar](https://github.com/rayyanhunerkar) for helping spark this idea to mind