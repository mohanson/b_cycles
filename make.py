import os
import subprocess

subprocess.call('cargo build --release', shell=True)
os.chdir('/src/pin-3.18-98332-gaebd7b1e6-gcc-linux/source/tools/ManualExamples')
subprocess.call('../../../pin -t obj-intel64/inscount0.so -- /src/b_cycles/target/release/b_cycles', shell=True)
r = subprocess.getoutput('cat inscount.out')
print(r, (int(r.split()[1]) - 700995) / (128 * 128))
