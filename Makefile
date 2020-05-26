
BUILTINDIR = builtin
STDDIR = stdlib

test: all
	cargo test

TESTEXES = $(BUILTINDIR)/kvstest.mexe $(STDDIR)/queuetest.mexe $(BUILTINDIR)/arraytest.mexe $(BUILTINDIR)/globaltest.mexe $(STDDIR)/priorityqtest.mexe $(STDDIR)/bytearraytest.mexe $(STDDIR)/keccaktest.mexe $(BUILTINDIR)/maptest.mexe
BUILTINMAOS = $(BUILTINDIR)/array.mao $(BUILTINDIR)/kvs.mao
STDLIBMAOS = $(STDDIR)/bytearray.mao $(STDDIR)/priorityq.mao $(STDDIR)/random.mao $(STDDIR)/queue.mao $(STDDIR)/keccak.mao
STDLIB = $(STDLIBMAOS)

all: $(TESTEXES)

$(BUILTINDIR)/kvstest.mexe: $(BUILTINMAOS) $(BUILTINDIR)/kvstest.mini
	cargo run compile $(BUILTINDIR)/kvstest.mini -o $(BUILTINDIR)/kvstest.mexe

$(STDDIR)/queuetest.mexe: $(BUILTINMAOS) $(STDDIR)/queuetest.mini $(STDLIB)
	cargo run compile $(STDDIR)/queuetest.mini $(STDLIB) -o $(STDDIR)/queuetest.mexe

$(BUILTINDIR)/arraytest.mexe: $(BUILTINMAOS) $(BUILTINDIR)/arraytest.mini
	cargo run compile $(BUILTINDIR)/arraytest.mini -o $(BUILTINDIR)/arraytest.mexe

$(BUILTINDIR)/globaltest.mexe: $(BUILTINMAOS) $(BUILTINDIR)/globaltest.mini
	cargo run compile $(BUILTINDIR)/globaltest.mini -o $(BUILTINDIR)/globaltest.mexe

$(STDDIR)/priorityqtest.mexe: $(BUILTINMAOS) $(STDDIR)/priorityqtest.mini $(STDLIB)
	cargo run compile $(STDDIR)/priorityqtest.mini $(STDLIB) -o $(STDDIR)/priorityqtest.mexe

$(STDDIR)/bytearraytest.mexe: $(BUILTINMAOS) $(STDDIR)/bytearraytest.mini $(STDLIB)
	cargo run compile $(STDDIR)/bytearraytest.mini $(STDLIB) -o $(STDDIR)/bytearraytest.mexe

$(STDDIR)/keccaktest.mexe: $(BUILTINMAOS) $(STDDIR)/keccaktest.mini $(STDDIR)/keccak.mao $(STDDIR)/bytearray.mao
	cargo run compile $(STDDIR)/keccaktest.mini $(STDDIR)/keccak.mao $(STDDIR)/bytearray.mao -o $(STDDIR)/keccaktest.mexe

$(STDDIR)/priorityq.mao: $(BUILTINMAOS) $(STDDIR)/priorityq.mini
	cargo run compile $(STDDIR)/priorityq.mini -c -o $(STDDIR)/priorityq.mao

$(STDDIR)/queue.mao: $(BUILTINMAOS) $(STDDIR)/queue.mini
	cargo run compile $(STDDIR)/queue.mini -c -o $(STDDIR)/queue.mao

$(STDDIR)/bytearray.mao: $(BUILTINMAOS) $(STDDIR)/bytearray.mini
	cargo run compile $(STDDIR)/bytearray.mini -c -o $(STDDIR)/bytearray.mao

$(STDDIR)/random.mao: $(STDDIR)/random.mini
	cargo run compile $(STDDIR)/random.mini -c -o $(STDDIR)/random.mao

$(STDDIR)/keccak.mao: $(STDDIR)/keccak.mini		
	cargo run compile $(STDDIR)/keccak.mini -c -o $(STDDIR)/keccak.mao

$(BUILTINDIR)/maptest.mexe: $(BUILTINMAOS) $(BUILTINDIR)/maptest.mini
	cargo run compile $(BUILTINDIR)/maptest.mini -o $(BUILTINDIR)/maptest.mexe

$(BUILTINDIR)/array.mao: $(BUILTINDIR)/array.mini
	cargo run compile $(BUILTINDIR)/array.mini -c -o $(BUILTINDIR)/array.mao

$(BUILTINDIR)/kvs.mao: $(BUILTINDIR)/kvs.mini
	cargo run compile $(BUILTINDIR)/kvs.mini -c -o $(BUILTINDIR)/kvs.mao

RUNTIMEDIR = arbruntime
RUNTIMEMAOS = $(RUNTIMEDIR)/accounts.mao $(RUNTIMEDIR)/messages.mao

runtime: $(RUNTIMEMAOS)

$(RUNTIMEDIR)/accounts.mao: $(RUNTIMEDIR)/accounts.mini
	cargo run compile $(RUNTIMEDIR)/accounts.mini -c -o $(RUNTIMEDIR)/accounts.mao

$(RUNTIMEDIR)/messages.mao: $(RUNTIMEDIR)/messages.mini
	cargo run compile $(RUNTIMEDIR)/messages.mini -c -o $(RUNTIMEDIR)/messages.mao

compiler: 
	cargo build

clean: 
	rm -f $(BUILTINMAOS) $(TESTEXES) $(STDLIBMAOS)