````
ffmpeg -framerate 100 -i capture/snapshot%06d.png -pix_fmt yuv420p -r 100 -s 1280x720 -c:v libx264 -b:v 5000k out.mp4
```

```
ffmpeg -framerate 244 -i capture/snapshot%06d.png -c:v libx264 -crf 0 -r 244 -preset veryslow -pix_fmt yuv420p out.mp4
```

```
convert -delay 0.02 -loop 0 *.png myimage.gif
```
