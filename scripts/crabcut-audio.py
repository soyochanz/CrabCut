#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Generate CrabCut's original CC0 starter sound pack, with no external samples."""
import hashlib, json, math, random, struct, wave
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1] / 'audio'
ROOT.mkdir(exist_ok=True)
RATE = 24000
rng = random.Random(42)
entries = []
def save(name, category, samples):
    peak = max(abs(v) for v in samples) or 1
    pcm = b''.join(struct.pack('<h', int(v / peak * 24000)) for v in samples)
    path = ROOT / (name + '.wav')
    with wave.open(str(path), 'wb') as out:
        out.setnchannels(1); out.setsampwidth(2); out.setframerate(RATE); out.writeframes(pcm)
    data = path.read_bytes()
    entries.append(dict(id=name, name=name.replace('-', ' ').title(), category=category,
        file=path.name, duration=len(samples)/RATE, bytes=len(data), license='CC0-1.0',
        source='https://github.com/soyochanz/CrabCut/blob/main/scripts/crabcut-audio.py',
        author='CrabCut procedural sound pack', sha256=hashlib.sha256(data).hexdigest()))
for name,seconds,pitch in [('quick-whoosh',.35,600),('soft-sweep',.7,220),('rising-sweep',1.2,900)]:
    last=0; samples=[]
    for i in range(int(RATE*seconds)):
        t=i/RATE; u=t/seconds
        last=.7*last+.3*rng.uniform(-1,1)
        samples.append((last+.15*math.sin(2*math.pi*(pitch*t+900*t*t)))*math.sin(math.pi*u)**2)
    save(name,'Whoosh' if seconds<1 else 'Riser',samples)
for name,seconds,pitch in [('deep-impact',.6,65),('pop-click',.12,440),('bright-tick',.09,1800),('soft-hit',.3,130)]:
    samples=[]
    for i in range(int(RATE*seconds)):
        t=i/RATE
        samples.append((math.sin(2*math.pi*(pitch*t+4*(1-math.exp(-30*t))))+.15*rng.uniform(-1,1))*math.exp(-9*t/seconds)*min(1,t/.004))
    save(name,'Impact',samples)
for name,bpm,notes in [('coral-groove',100,[48,55,57,53]),('midnight-pulse',120,[45,53,48,55])]:
    beat=60/bpm; seconds=beat*32; samples=[]
    for i in range(round(RATE*seconds)):
        t=i/RATE; step=int(t/beat); phase=t%beat
        root=notes[(step//8)%len(notes)]; f=440*2**((root-69)/12)
        kick=math.sin(2*math.pi*(48*phase+1.8*(1-math.exp(-35*phase))))*math.exp(-22*phase)
        bass=.3*math.sin(2*math.pi*f*t)*math.exp(-4*phase)
        arp_phase=t%(beat/2); arp=440*2**((root+12+[0,4,7,12][int(t/(beat/2))%4]-69)/12)
        tone=.16*(math.sin(2*math.pi*arp*t)+.2*math.sin(4*math.pi*arp*t))*math.exp(-14*arp_phase)*min(1,arp_phase/.008)
        hat=.06*rng.uniform(-1,1)*math.exp(-90*arp_phase)
        snare=.12*rng.uniform(-1,1)*math.exp(-25*phase) if step%2 else 0
        fade=min(1,t/.02,(seconds-t)/.02)
        samples.append((.45*kick+bass+tone+hat+snare)*fade)
    save(name,'Music',samples)
(ROOT/'manifest.json').write_text(json.dumps(dict(version=1,sounds=entries),indent=2)+'\n')
(ROOT/'LICENSE.txt').write_text('CrabCut procedural audio pack. These generated audio files are dedicated to the public domain under CC0 1.0. No external recordings or samples are used. https://creativecommons.org/publicdomain/zero/1.0/\n')
print(f'Generated {len(entries)} sounds')
