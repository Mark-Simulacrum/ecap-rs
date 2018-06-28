#define LIBECAP_VERSION "1.0.1"

#include <iostream>
#include <libecap/common/forward.h>
#include <libecap/common/registry.h>
#include <libecap/common/errors.h>
#include <libecap/adapter/service.h>
#include <libecap/adapter/xaction.h>
#include <libecap/host/xaction.h>
#include <libecap/host/host.h>
#include <libecap/common/name.h>
#include <libecap/common/named_values.h>
#include <libecap/common/message.h>
#include <libecap/common/header.h>
#include <libecap/common/version.h>
#include <libecap/common/body.h>
#include <libecap/common/body_size.h>
#include <libecap/common/memory.h>
#include <libecap/common/delay.h>
#include <sys/time.h>

struct RustLogVerbosity {
    size_t mask;
};

struct pstr {
    size_t size;
    const char *buf;
};

struct rust_string {
    size_t size;
    const char *buf;
    size_t capacity;
};

struct rust_version {
    int majr;
    int minr;
    int micr;
};

#define DETAILS_SIZE 16
#define DETAILS_ALIGN 8

struct rust_details__ {
    char details[DETAILS_SIZE];
    uint64_t __align[0];
};

struct rust_area {
    size_t size;
    const char *buf;
    rust_details__ details;
};

// We'll be memcpying the raw bytes in here to preserve them across the C boundary
static_assert(sizeof(libecap::Area::Details) <= DETAILS_SIZE);
static_assert(alignof(libecap::Area::Details) == DETAILS_ALIGN);
static_assert(alignof(rust_details__) == DETAILS_ALIGN);

#define NAME_SIZE 40
#define NAME_ALIGN 8

struct rust_name {
    char name[NAME_SIZE];
    uint64_t __align[0];
};

// We'll be memcpying the raw bytes in here to preserve them across the C boundary
static_assert(sizeof(libecap::Name) <= NAME_SIZE);
static_assert(sizeof(rust_name) == NAME_SIZE);
static_assert(alignof(libecap::Name) == NAME_ALIGN);
static_assert(alignof(rust_name) == NAME_ALIGN);

rust_name to_rust_name(libecap::Name name) {
    rust_name foo;
    auto name_ptr = new (&foo.name) libecap::Name;
    *name_ptr = name;
    return foo;
}

const libecap::Name *from_rust_name(const rust_name *name) {
    return reinterpret_cast<const libecap::Name*>(name);
}

libecap::Name *from_rust_name(rust_name *name) {
    return reinterpret_cast<libecap::Name*>(name);
}

rust_area to_rust_area(libecap::Area area) {
    rust_area foo;
    foo.size = area.size;
    foo.buf = area.start;
    auto details = new (&foo.details) libecap::Area::Details;
    *details = area.details;
    return foo;
}

#define SHARED_PTR_MESSAGE_SIZE 16

struct rust_shared_ptr_message {
    char value[SHARED_PTR_MESSAGE_SIZE];
    uint64_t __align[0];
};

// We'll be memcpying the raw bytes in here to preserve them across the C boundary
static_assert(sizeof(libecap::shared_ptr<libecap::Message>) <= SHARED_PTR_MESSAGE_SIZE);
static_assert(alignof(libecap::shared_ptr<libecap::Message>) == alignof(rust_shared_ptr_message));
static_assert(alignof(libecap::shared_ptr<libecap::Message>) == 8);

rust_shared_ptr_message to_rust_shared_ptr_message(libecap::shared_ptr<libecap::Message> msg) {
    rust_shared_ptr_message foo;
    auto ptr = new (&foo.value) libecap::shared_ptr<libecap::Message>;
    *ptr = msg;
    return foo;
}

extern "C" const libecap::Message *rust_shim_shared_ptr_message_ref(
        const libecap::shared_ptr<libecap::Message> *msg) {
    return msg->get();
}

extern "C" libecap::Message *rust_shim_shared_ptr_message_ref_mut(
        libecap::shared_ptr<libecap::Message> *msg) {
    return msg->get();
}

extern "C" void rust_shim_shared_ptr_message_free(libecap::shared_ptr<libecap::Message> *msg) {
    msg->reset();
}

extern "C" {
    rust_string rust_new_string(const char*, size_t);
    void rust_free_string(rust_string);
    void rust_service_describe(const void **, const void *);
    rust_string rust_service_uri(const void **);
    rust_string rust_service_tag(const void **);
    void rust_service_start(const void **);
    void rust_service_stop(const void **);
    void rust_service_retire(const void **);
    bool rust_service_is_async(const void **);
    void rust_service_resume(const void **);
    void rust_service_suspend(const void **, timeval *);
    void rust_service_configure(const void **, const libecap::Options *);
    void rust_service_reconfigure(const void **, const libecap::Options *);
    bool rust_service_wants_url(const void **, const char *);

    void rust_xaction_start(const void *);
    void rust_xaction_stop(const void *);
    void rust_xaction_resume(const void *);
    void rust_xaction_ab_discard(const void *);
    void rust_xaction_ab_make(const void *);
    void rust_xaction_ab_make_more(const void *);
    void rust_xaction_ab_stop_making(const void *);
    void rust_xaction_ab_pause(const void *);
    void rust_xaction_ab_resume(const void *);
    rust_area rust_xaction_ab_content(const void *, size_t, size_t);
    void rust_xaction_ab_content_shift(const void *, size_t);

    void rust_xaction_vb_content_done(const void *, bool);
    void rust_xaction_vb_content_available(const void *);

    void rust_service_free(const void **);
    const void *rust_xaction_create(const void **, void *);
    void rust_xaction_free(const void *);
}

rust_string to_rust_string(const std::string &s) {
    // note that this incurs a copy cost since we're transferring ownership to rust
    return rust_new_string(s.data(), s.length());
}

// TODO: This will copy the std::string returned by uri
extern "C" rust_string rust_shim_host_uri() {
    return to_rust_string(libecap::MyHost().uri());
}

extern "C" void *rust_shim_host_open_debug(RustLogVerbosity verbosity) {
    return libecap::MyHost().openDebug(libecap::LogVerbosity(verbosity.mask));
}

extern "C" void rust_shim_host_close_debug(std::ostream* stream) {
    libecap::MyHost().closeDebug(stream);
}

extern "C" void rust_shim_ostream_write(std::ostream *stream, char *buf, size_t length) {
    stream->write(buf, length);
}

extern "C" libecap::host::Host &rust_host() {
    return libecap::MyHost();
}

extern "C" rust_area rust_area_new() {
    auto area = libecap::Area();
    rust_area foo;
    foo.size = area.size;
    foo.buf = area.start;
    auto details = new (&foo.details) libecap::Area::Details;
    *details = area.details;
    return foo;
}

extern "C" rust_area rust_area_new_slice(char *buf, size_t len) {
    auto area = libecap::Area::FromTempBuffer(buf, len);
    rust_area foo;
    foo.size = area.size;
    foo.buf = area.start;
    auto details = new (&foo.details) libecap::Area::Details;
    *details = area.details;
    return foo;
}

extern "C" void rust_area_free(rust_area *area) {
    libecap::Area::Details& details =
        reinterpret_cast<libecap::Area::Details&>(area->details);
    details.reset();
}

namespace Adapter { // not required, but adds clarity

class Service: public libecap::adapter::Service {
	public:
		// About
		virtual std::string uri() const; // unique across all vendors
		virtual std::string tag() const; // changes with version and config
		virtual void describe(std::ostream &os) const; // free-format info
		virtual bool makesAsyncXactions() const;

		// Configuration
		virtual void configure(const libecap::Options &cfg);
		virtual void reconfigure(const libecap::Options &cfg);

		// Lifecycle
		virtual void start(); // expect makeXaction() calls
		virtual void stop(); // no more makeXaction() calls until start()
		virtual void retire(); // no more makeXaction() calls
		virtual void suspend(timeval &timeout); // influence host waiting time
		virtual void resume(); // kick async xactions via host::Xaction::resume

		// Scope
		virtual bool wantsUrl(const char *url) const;

		// Work
		virtual MadeXactionPointer makeXaction(libecap::host::Xaction *hostx);

		Service(const void **rust_service);
		~Service();
	public:
	    // Rust service, unknown; managed on Rust's end
	    const void **rust_service;
};

// a minimal adapter transaction
class Xaction: public libecap::adapter::Xaction {
	public:
		Xaction(Service *service, libecap::host::Xaction *x);
		virtual ~Xaction();

		// meta-info for the host transaction
		virtual const libecap::Area option(const libecap::Name &name) const;
		virtual void visitEachOption(libecap::NamedValueVisitor &visitor) const;

		// lifecycle
		virtual void start();
		virtual void stop();
		virtual void resume();

		// adapted body transmission control
		virtual void abDiscard();
		virtual void abMake();
		virtual void abMakeMore();
		virtual void abStopMaking();
		virtual void abPause();
		virtual void abResume();

		// adapted body content extraction and consumption
		virtual libecap::Area abContent(libecap::size_type, libecap::size_type);
		virtual void abContentShift(libecap::size_type);

		// virgin body state notification
		virtual void noteVbContentDone(bool);
		virtual void noteVbContentAvailable();

	private:
		const void *rust_xaction;
};

} // namespace Adapter

std::string Adapter::Service::uri() const {
    rust_string s = ::rust_service_uri(rust_service);
    std::string ret = std::string(s.buf, s.size);
    ::rust_free_string(s);
    return ret;
}

std::string Adapter::Service::tag() const {
    rust_string s = ::rust_service_tag(rust_service);
    std::string ret = std::string(s.buf, s.size);
    ::rust_free_string(s);
    return ret;
}

void Adapter::Service::describe(std::ostream &os) const {
    ::rust_service_describe(rust_service, &os);
}

extern "C" rust_area options_option(const libecap::Options *options, const char* buf, size_t len) {
    Must(options);
    libecap::Name name = libecap::Name(std::string(buf, len));
    libecap::Area area = options->option(name);
    rust_area foo;
    foo.size = area.size;
    foo.buf = area.start;
    auto details = new (&foo.details) libecap::Area::Details;
    *details = area.details;
    return foo;
}

typedef void (*visitor_callback)(const rust_name *, const char*, size_t, void*);

class NamedValueVisitorImpl: public libecap::NamedValueVisitor {
    public:
        NamedValueVisitorImpl(visitor_callback a_callback, void* an_extra):
            callback(a_callback), extra(an_extra) {};
        virtual void visit(const libecap::Name &name, const libecap::Area &value) {
            callback((const rust_name*) &name, value.start, value.size, extra);
        }
        visitor_callback callback;
        void* extra;
};

extern "C" void options_visit(
        const libecap::Options *options, visitor_callback callback, void* extra) {
    Must(options);
    auto visitor = NamedValueVisitorImpl(callback, extra);
    options->visitEachOption(visitor);
}

bool Adapter::Service::makesAsyncXactions() const {
	return rust_service_is_async(rust_service);
}

void Adapter::Service::configure(const libecap::Options &options) {
    rust_service_configure(rust_service, &options);
}

void Adapter::Service::reconfigure(const libecap::Options &options) {
    rust_service_reconfigure(rust_service, &options);
}

void Adapter::Service::start() {
	libecap::adapter::Service::start();
	rust_service_start(rust_service);
}

void Adapter::Service::suspend(timeval &delay) {
	rust_service_suspend(rust_service, &delay);
}

void Adapter::Service::stop() {
	rust_service_stop(rust_service);
	libecap::adapter::Service::stop();
}

void Adapter::Service::resume() {
	rust_service_resume(rust_service);
	libecap::adapter::Service::stop();
}

void Adapter::Service::retire() {
	rust_service_retire(rust_service);
}

bool Adapter::Service::wantsUrl(const char *url) const {
    return rust_service_wants_url(rust_service, url);
}

Adapter::Service::MadeXactionPointer
Adapter::Service::makeXaction(libecap::host::Xaction *hostx) {
	return Adapter::Service::MadeXactionPointer(new Adapter::Xaction(this, hostx));
}


Adapter::Xaction::Xaction(Adapter::Service *service, libecap::host::Xaction *x) {
    rust_xaction = rust_xaction_create(service->rust_service, x);
}

Adapter::Xaction::~Xaction() {
	rust_xaction_free(rust_xaction);
	rust_xaction = nullptr;
}

const libecap::Area Adapter::Xaction::option(const libecap::Name &) const {
	return libecap::Area(); // this transaction has no meta-information
}

void Adapter::Xaction::visitEachOption(libecap::NamedValueVisitor &) const {
	// this transaction has no meta-information to pass to the visitor
}

libecap::Area Adapter::Xaction::abContent(libecap::size_type offset, libecap::size_type size) {
    rust_area rarea = ::rust_xaction_ab_content(rust_xaction, offset, size);
    libecap::Area area;
    area.start = rarea.buf;
    area.size = rarea.size;
    auto details = new (&area.details) libecap::Area::Details;
    *details = reinterpret_cast<libecap::Area::Details&>(rarea.details);
    rust_area_free(&rarea);
    return area;
}

void Adapter::Xaction::abContentShift(libecap::size_type size) {
    ::rust_xaction_ab_content_shift(rust_xaction, size);
}

void Adapter::Xaction::noteVbContentDone(bool atEnd) {
    ::rust_xaction_vb_content_done(rust_xaction, atEnd);
}

#define XACTION_METHOD_SHIM(rname__, cpp_name__) \
    void Adapter::Xaction::cpp_name__() { \
        ::rname__(rust_xaction); \
    }

XACTION_METHOD_SHIM(rust_xaction_start, start);
XACTION_METHOD_SHIM(rust_xaction_resume, resume);
XACTION_METHOD_SHIM(rust_xaction_stop, stop);
XACTION_METHOD_SHIM(rust_xaction_ab_discard, abDiscard);
XACTION_METHOD_SHIM(rust_xaction_ab_make, abMake);
XACTION_METHOD_SHIM(rust_xaction_ab_make_more, abMakeMore);
XACTION_METHOD_SHIM(rust_xaction_ab_stop_making, abStopMaking);
XACTION_METHOD_SHIM(rust_xaction_ab_pause, abPause);
XACTION_METHOD_SHIM(rust_xaction_ab_resume, abResume);
XACTION_METHOD_SHIM(rust_xaction_vb_content_available, noteVbContentAvailable);

extern "C" void rust_shim_host_xaction_use_virgin(libecap::host::Xaction *xaction) {
    xaction->useVirgin();
}

extern "C" void rust_shim_host_xaction_block_virgin(libecap::host::Xaction *xaction) {
    xaction->blockVirgin();
}

extern "C" void rust_shim_host_xaction_adaptation_aborted(libecap::host::Xaction *xaction) {
    xaction->adaptationAborted();
}

extern "C" void rust_shim_host_xaction_adaptation_delayed(libecap::host::Xaction *xaction,
        const char* state, size_t len, double progress) {
    xaction->adaptationDelayed(libecap::Delay(std::string(state, len), progress));
}

extern "C" void rust_shim_host_xaction_resume(libecap::host::Xaction *xaction) {
    xaction->resume();
}

extern "C" void rust_shim_host_xaction_vb_make(libecap::host::Xaction *xaction) {
    xaction->vbMake();
}

extern "C" void rust_shim_host_xaction_vb_discard(libecap::host::Xaction *xaction) {
    xaction->vbDiscard();
}

extern "C" void rust_shim_host_xaction_vb_pause(libecap::host::Xaction *xaction) {
    xaction->vbDiscard();
}

extern "C" void rust_shim_host_xaction_vb_resume(libecap::host::Xaction *xaction) {
    xaction->vbResume();
}

extern "C" void rust_shim_host_xaction_vb_make_more(libecap::host::Xaction *xaction) {
    xaction->vbMakeMore();
}

extern "C" void rust_shim_host_xaction_vb_stop_making(libecap::host::Xaction *xaction) {
    xaction->vbStopMaking();
}

extern "C" rust_area rust_shim_host_xaction_vb_content(libecap::host::Xaction *xaction, size_t offset, size_t size) {
    libecap::Area area = xaction->vbContent(offset, size);
    rust_area foo;
    foo.size = area.size;
    foo.buf = area.start;
    auto details = new (&foo.details) libecap::Area::Details;
    *details = area.details;
    return foo;
}

extern "C" void rust_shim_host_xaction_vb_content_shift(libecap::host::Xaction *xaction, size_t size) {
    xaction->vbContentShift(size);
}

Adapter::Service::Service(const void **serv) {
    rust_service = serv;
}

Adapter::Service::~Service() {
    rust_service_free(rust_service);
}

extern "C" rust_name rust_name_new_unknown() {
    return to_rust_name(libecap::Name());
}

extern "C" rust_name rust_name_new_image(const char *buf, size_t len) {
    return to_rust_name(libecap::Name(std::string(buf, len)));
}

extern "C" rust_name rust_name_new_image_id(const char *buf, size_t len, int id) {
    return to_rust_name(libecap::Name(std::string(buf, len), id));
}

extern "C" bool rust_name_identified(rust_name* rname) {
    return from_rust_name(rname)->identified();
}

extern "C" bool rust_name_known(rust_name* rname) {
    return from_rust_name(rname)->known();
}

extern "C" bool rust_name_eq(rust_name *ra, rust_name *rb) {
    const libecap::Name &a = *from_rust_name(ra);
    const libecap::Name &b = *from_rust_name(rb);
    return a == b;
}

extern "C" pstr rust_name_image(rust_name *a) {
    const auto &image = from_rust_name(a)->image();
    return pstr {
        size: image.size(),
        buf: image.data(),
    };
}

extern "C" void rust_name_free(rust_name *name) {
    libecap::Name cpp_name = *reinterpret_cast<libecap::Name*>(name->name);
    // XXX: test that this actually works to run name's dtor
}

extern "C" rust_shared_ptr_message rust_shim_message_clone(const libecap::Message *msg) {
    return to_rust_shared_ptr_message(msg->clone());
}

extern "C" const libecap::FirstLine *rust_shim_message_first_line(const void *in_msg) {
    const auto *msg = reinterpret_cast<const libecap::Message*>(in_msg);
    return &msg->firstLine();
}

extern "C" libecap::FirstLine *rust_shim_message_first_line_mut(void *in_msg) {
    auto *msg = reinterpret_cast<libecap::Message*>(in_msg);
    return &msg->firstLine();
}

extern "C" const libecap::Body *rust_shim_message_body(const libecap::Message *msg) {
    return msg->body();
}

extern "C" libecap::Body *rust_shim_message_body_mut(libecap::Message *msg) {
    return msg->body();
}

extern "C" const libecap::Header *rust_shim_message_header(const libecap::Message *msg) {
    return &msg->header();
}

extern "C" libecap::Header *rust_shim_message_header_mut(libecap::Message *msg) {
    return &msg->header();
}

extern "C" void rust_shim_host_xaction_use_adapted(libecap::host::Xaction *xaction, libecap::shared_ptr<libecap::Message> *adapted) {
    xaction->useAdapted(*adapted);
}

extern "C" libecap::Message *rust_shim_host_xaction_virgin(libecap::host::Xaction *xaction) {
    return &xaction->virgin();
}

extern "C" const libecap::Message *rust_shim_host_xaction_cause(libecap::host::Xaction *xaction) {
    return &xaction->cause();
}

extern "C" libecap::Message *rust_shim_host_xaction_adapted(libecap::host::Xaction *xaction) {
    return &xaction->adapted();
}

extern "C" void rust_shim_host_xaction_note_ab_content_available(libecap::host::Xaction *xaction) {
    xaction->noteAbContentAvailable();
}

extern "C" void rust_shim_host_xaction_note_ab_content_done(libecap::host::Xaction *xaction, bool end) {
    xaction->noteAbContentDone(end);
}


struct body_size {
    bool known;
    uint64_t size;
};

extern "C" body_size rust_shim_body_size(libecap::Body *body) {
    auto size = body->bodySize();
    if (size.known()) {
        return body_size {
            known: true,
            size: size.value(),
        };
    } else {
        return body_size {
            known: false,
            size: 0,
        };
    }
}

extern "C" rust_version rust_shim_version(const void *first_line) {
    const auto *msg = reinterpret_cast<const libecap::FirstLine*>(first_line);
    const auto version = msg->version();
    return rust_version {
        majr: version.majr,
        minr: version.minr,
        micr: version.micr,
    };
}

extern "C" void rust_shim_set_version(libecap::FirstLine *first_line, const rust_version *version) {
    first_line->version(libecap::Version(version->majr, version->minr, version->micr));
}

extern "C" void rust_shim_header_has_any(const libecap::Header *header, const rust_name* name) {
    header->hasAny(*from_rust_name(name));
}

extern "C" rust_area rust_shim_header_value(const libecap::Header *header, const rust_name* name) {
    return to_rust_area(header->value(*from_rust_name(name)));
}

extern "C" void rust_shim_header_add(libecap::Header *header, const rust_name* name, const rust_area* value) {
    auto area = libecap::Area(value->buf, value->size);
    header->add(*from_rust_name(name), area);
}

extern "C" void rust_shim_header_remove_any(libecap::Header *header, const rust_name* name) {
    header->removeAny(*from_rust_name(name));
}

extern "C" rust_area rust_shim_header_image(libecap::Header *header) {
    return to_rust_area(header->image());
}

extern "C" void rust_shim_header_parse(libecap::Header *header, const rust_area *buf) {
    auto area = libecap::Area(buf->buf, buf->size);
    header->parse(area);
}

extern "C" void rust_shim_header_visit_each(
        const libecap::Header *header, visitor_callback callback, void* extra) {
    auto visitor = NamedValueVisitorImpl(callback, extra);
    header->visitEach(visitor);
}

extern "C" bool rust_shim_register_service(const void **service) {
    return libecap::RegisterVersionedService(new Adapter::Service(service));
}
