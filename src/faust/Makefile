define dsp2rs
	faust -lang rust -a template.rs -o auto/$(2).rs -cn $(2) dsp/$(1).dsp 
endef

all:
	$(call dsp2rs,os.osc,OsOsc)
	$(call dsp2rs,no.noise,NoNoise) 

clean:
	rm -rf auto/*.rs

