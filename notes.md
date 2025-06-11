# distribution
dst-cc and how it would be used do create a d-bus build system and rebuilder d
dfeet for debugging dbus

# threat model 
software attacks like orion and trust zone mitigations
- CVE-2020-10148 (Orion) 9.8
- CVE-2024-23917 (JetBrains) 9.8

Both of these seem to be related to http or api verification to build servers. This allows attackers to gain access to a build server and alter code before updates are pushed to production and distributed to users. The goal of this project is to mitigate the ability for malicious actors to embed undetected malware into trusted software products. 

### threat
  If a user is able to gain access to a build server, like the Orion build servers or the JetBrains TeamCity servers, they can insert malicious code into trusted programs and sign binaries so they are virtually indetectable (at least hidden from any anti-malware). This can result in malware being distributed to user's computers through trusted programs with unknowingly malicious code.

  A malicious actor may also be able to alter signing keys if they are not stored properly. This would allow an attacker to esure their malware does not get detected when a program is compiled or distributed. This causes the same threat of malware being distributed within trusted programs. 

### accessibility 
  These attacks require vulnerabilities in build server verification methods, allowing malicious actors like APTs to gain access to the servers to insert malicious code into trusted programs. Unfortunately, exploits like this are not uncommmon and, with a lack of further protections, critical vulnerabilities as they allow for malware to be widely distributed and undetectable. 

### mitigation
  To prevent injection of insecure code an ephemeral build environment can be created where a build environment is flashed (and verified through secure boot) to a trustzone enabled device and binaries are compiled and signed in the secure world before wiping the entire device. By keeping the verification/signing keys in the secure world or immutable storage, binaries can be signed and compared against known-good builds innaccessbile to attackers.  

### result
  This environment would prevent attackers who have gained access to build servers from modifying the build envrionment and binary verification as build firmware can be verified independently before use, esuring it matches known-good compilation firmware. This can also ensure tha created binaries are signed in the secure world, ensuring that the signature cannot be modified by attackers who gained access to build servers. 

  In the case that an attacker is able to gain access to a secure build environment and insert malware into a binary during the build process, the use of an ephemeral system will help prevent persistant threats, reducing the serverity of the attack. 

# device plan
  - Host contains worker node build firmware/build scripts in the secure world.
  - Armories are connected to host with no existing firmware, only a secure boot key burned to OTP memory.
  - Armories are flashed firmware through serial from the host, firmware is verified with secure boot and started.
  - Armory firmware waits for code to be compiled, then builds in secure world and signs binary before sending back to host for verification (this verification may happen on the armory)
  - Armory firmware is ephemeral

### execution pattern
  1) Host recieves message from insecure world to compile given code base
  2) Host provisions portions of code to be compiled by each worker node (usbarmory) using dstcc
  3) Host secure world flashes pre-built and signed build firmware to each necessary worker node
  4) Worker nodes verfiy flashed firmware using secure boot and a universal OTP key (likely best practice to be different per device)
  5) Worker nodes then wait for network cdc bulk transfer of code to be compiled.
  6) Host insecure world sends code to worker nodes to be compiled
  7) Worker nodes recieve code in insecure world
  8) Worker nodes compile binaries in secure world
  9) Worker nodes sign binaries in secure world
  9) Worker nodes send signed binaries to host
  10) Host secure world verifies signed binaries with keys stored in the secure world (immutable from users in insecure world).
  11) Final signed and verified binary sent back to user in insecure world. 

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