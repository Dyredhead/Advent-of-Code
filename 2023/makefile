all:
	make compile && make run && make clean
compile $(ARGS): $(ARGS).rs $(ARGS).txt  
	rustc $(ARGS).rs
run: $(ARGS).rs
	./$(ARGS)
clean: $(ARGS)
	rm $(ARGS)
