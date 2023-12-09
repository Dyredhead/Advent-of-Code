all:
	make compile && make run && make clean
compile Day_$(DAY): Day_$(DAY).rs Day_$(DAY).txt  
	rustc Day_$(DAY).rs
run: Day_$(DAY).rs
	./Day_$(DAY)
clean: Day_$(DAY)
	rm Day_$(DAY)
