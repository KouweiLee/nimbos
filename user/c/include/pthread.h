#ifndef __PTHREAD_H__
#define __PTHREAD_H__

typedef unsigned long pthread_t;

int pthread_create(pthread_t *res, int (*entry)(void *), void *arg);

#endif // __PTHREAD_H__
