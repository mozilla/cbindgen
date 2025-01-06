#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef const char *Str;

typedef void (*SetCallback)(Str key);

typedef void (*MapCallback)(Str key, uint64_t val);

typedef struct {
  uintptr_t num_buckets;
  uintptr_t capacity;
  uint8_t *occupied;
  Str *keys;
  char *vals;
} HashTable_Str__c_char__false;

typedef HashTable_Str__c_char__false MySet;

typedef struct {
  uintptr_t num_buckets;
  uintptr_t capacity;
  uint8_t *occupied;
  Str *keys;
  uint64_t *vals;
} HashTable_Str__u64__true;

MySet *new_set(void);

void set_for_each(const MySet *set, SetCallback callback);

HashTable_Str__u64__true *new_map(void);

void map_for_each(const HashTable_Str__u64__true *map, MapCallback callback);
