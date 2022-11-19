PREFIX=/usr/local
DESTDIR=
CC?=/usr/local/bin/gcc-11
VERSION?=0.3
DISTNAME=NGmerge-${VERSION}
CFLAGS?=-fopenmp

NGmerge: NGmerge.c NGmerge.h
	mkdir -p build
	/usr/local/bin/gcc-11 -g -Wall -std=gnu99 -O2 -c -lomp -o ngmerge.o NGmerge.c -lz ${LDFLAGS}
	ar rcs build/libngmerge.a ngmerge.o

install: NGmerge
	@mkdir -p $(DESTDIR)$(PREFIX)/bin
	cp NGmerge $(DESTDIR)$(PREFIX)/bin

clean:
	-@rm NGmerge 2>/dev/null || true
	-@rm -rf ${DISTNAME}*

dist:
	rm -rf ${DISTNAME}*
	mkdir ${DISTNAME}
	cp -r `ls | grep -v ${DISTNAME}` ${DISTNAME}
	tar czvf ${DISTNAME}.tar.gz ${DISTNAME}
