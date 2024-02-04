# Directories
BUILD_DIR = build
OBJ_DIR = $(BUILD_DIR)/obj

# Complation
CC=gcc

# Files
TARGET = $(BUILD_DIR)/vimail

OBJECTS = $(OBJ_DIR)/main.o \
		  $(OBJ_DIR)/draw.o \
		  $(OBJ_DIR)/tools.o \
		  $(OBJ_DIR)/control.o 

HEADERS = draw.h \
		  tools.h \
		  control.h

# ----------------------------
#  Rules
default: $(TARGET)

# Build binary
$(TARGET): $(OBJECTS)
	@mkdir -p $(BUILD_DIR)
	$(CC) $^ -o $@

# Build object files
$(OBJ_DIR)/%.o: %.c $(HEADERS)
	@mkdir -p $(OBJ_DIR) 
	$(CC) -c -o $@ $<

clean:
	rm -rf build
