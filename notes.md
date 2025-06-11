# distribution
dst-cc and how it would be used do create a d-bus build system and rebuilder d
dfeet for debugging dbus

# threat model 
software attacks like orion and trust zone mitigations
- CVE-2020-10148 (Orion)
- CVE-2024-23917 (JetBrains)

Both of these seem to be related to http or api verification to build servers. This allows attackers to gain access to a build server and alter code before updates are pushed to production and distributed to users. The goal of this project is to mitigate the ability for malicious actors to embed undetected malware into trusted software products. 

# tamago
tamago qemu build example -- example built for x86, 

start qemu server (command in ~/research/tamago-example/): 
```bash
sudo qemu-system-x86_64 \
  -machine microvm,x-option-roms=on,pit=off,pic=off,rtc=on \
  -global virtio-mmio.force-legacy=false \
  -enable-kvm \
  -cpu host,invtsc=on,kvmclock=on \
  -no-reboot \
  -m 4G \
  -nographic \
  -monitor none \
  -serial stdio \
  -device virtio-net-device,netdev=net0 \
  -netdev tap,id=net0,ifname=tap0,script=no,downscript=no \
  -kernel example
```
enter server (command in ~/research/tamago-example/):
```bash
ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no 10.0.0.1
```
example.imx in flashing tools is a proper tamago bare-metal kernal that can be flashed to the usb-armory. It was compiled using make imx TARGET=usbarmory in the ~/research/tamago/tamago-example directory. Note, for some reason compiling the tamago compiler does not work properly, so you have to use a pre-built compiler, idk why but I don't think that is super important. then simply connect using `ssh user@10.0.0.1`. (you may need to run `ssh-keygen -f '/home/william/.ssh/known_hosts' -R '10.0.0.1'`)

these commands must be run first: 
```bash
sudo ip addr add 10.0.0.2/24 dev enx1a5589a26942
sudo ip link set enx1a5589a26942 up
```

device communication through network device ssh