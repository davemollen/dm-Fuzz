from scipy import signal
import numpy as np
import matplotlib.pyplot as plt

# Set the sample rate
sample_rate = 44100  # in Hz

# Change the filter value to see the difference in the frequency response
# Keep it between 0 and 1
filter = 0.5
filter = 1. - filter
filter = max(filter * filter * 0.175438596, 0.00001)

def get_s_domain_coefficients(t):
  # ( (1.-t)*R3 * C1 * t*R3 + (1.-t)*R3 * C1 * R2 ) * s + ( (1.-t)*R3 + t*R3 )
  # ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  # ( (1.-t)*R3 * C1 * t*R3 + (1.-t)*R3 * C1 * R2 + C1 * R1 * t*R3 + C1 * R1 * R2 ) * s + ( (1.-t)*R3 + t*R3 + R2 + R1 )

  r1 = 10000
  r2 = 47000
  r3 = 100000
  c1 = 2.2e-8

  r3_a = (1.-t)*r3
  r3_b = t*r3

  b0 = r3_b * c1 * r3_a + r3_b * c1 * r2
  b1 = r3
  a0 = b0 + c1 * r1 * r3_a + c1 * r1 * r2
  a1 = b1 + r2 + r1

  return ([b0, b1], [a0, a1])

[num, den] = get_s_domain_coefficients(filter)
print("s-domain coefficients:", (num, den))

# Apply the bilinear transform
b, a = signal.bilinear(num, den, fs=sample_rate)
print("z-domain coefficients:", (list(b), list(a)))

# Get the frequency response
w,h = signal.freqz(b, a, 2**20)
w = w * sample_rate / (2 *np.pi)

# Plot the frequency response
fig1 = plt.figure(1)
plt.title('Digital filter frequency response')
plt.semilogx(w, 20 * np.log10(abs(h)), 'b')
plt.ylabel('magnitude [dB]')
plt.xlabel('frequency [Hz]')
plt.grid()
plt.axis('tight')
plt.xlim([10, 20000])
plt.show()