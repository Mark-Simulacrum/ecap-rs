#define LIBECAP_VERSION "1.0.1"

#include <iostream>
#include <sstream>
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
#include <climits>

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

// XXX This is intended as a one-to-one copy of the libecap::Name class to bypass `private`
struct cpp_name {
    std::string image_;
    int id_;
    int hostId_;
};

static_assert(sizeof(cpp_name) == sizeof(libecap::Name));
static_assert(alignof(cpp_name) == alignof(libecap::Name));

struct rust_name {
    pstr image;
    int id;
    int host_id;
};

// The returned rust_name contains pointers into the passed name and must not outlive name.
rust_name to_rust_name(const libecap::Name &name) {
    const cpp_name &namef = reinterpret_cast<const cpp_name &>(name);
    return rust_name {
        image: pstr {
            size: namef.image_.size(),
            buf: namef.image_.data(),
        },
        id: namef.id_,
        host_id: namef.hostId_,
    };
}

libecap::Name from_rust_name(const rust_name *name) {
    auto cpp = cpp_name {
        image_: std::string(name->image.buf, name->image.size),
        id_: name->id,
        hostId_: name->host_id,
    };
    return *reinterpret_cast<libecap::Name*>(&cpp);
}

extern "C" void rust_area_free(rust_area *area) {
    libecap::Area::Details& details =
        reinterpret_cast<libecap::Area::Details&>(area->details);
    details.reset();
}

libecap::Area from_rust_area(rust_area area) {
    libecap::Area foo;
    foo.start = area.buf;
    foo.size = area.size;
    auto details = new (&foo.details) libecap::Area::Details;
    *details = reinterpret_cast<libecap::Area::Details&>(area.details);
    rust_area_free(&area);
    return foo;
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

    void rust_xaction_start(const void *, void *);
    void rust_xaction_stop(const void *, void *);
    void rust_xaction_resume(const void *, void *);
    void rust_xaction_ab_discard(const void *, void *);
    void rust_xaction_ab_make(const void *, void *);
    void rust_xaction_ab_make_more(const void *, void *);
    void rust_xaction_ab_stop_making(const void *, void *);
    void rust_xaction_ab_pause(const void *, void *);
    void rust_xaction_ab_resume(const void *, void *);
    rust_area rust_xaction_ab_content(const void *, void *, size_t, size_t);
    void rust_xaction_ab_content_shift(const void *, void *, size_t);

    void rust_xaction_vb_content_done(const void *, void *, bool);
    void rust_xaction_vb_content_available(const void *, void *);

    void rust_service_free(const void **);
    const void *rust_xaction_create(const void **, void *);
    void rust_xaction_free(const void *);
}

rust_string to_rust_string(const std::string &s) {
    // note that this incurs a copy cost since we're transferring ownership to rust
    return rust_new_string(s.data(), s.length());
}

// TODO: This will copy the std::string returned by uri
extern "C" rust_string rust_shim_host_uri(const libecap::host::Host *host) {
    return to_rust_string(host->uri());
}

extern "C" rust_string rust_shim_host_describe(const libecap::host::Host *host) {
    std::ostringstream out;
    host->describe(out);
    return to_rust_string(out.str());
}

extern "C" void *rust_shim_host_open_debug(libecap::host::Host *host, RustLogVerbosity verbosity) {
    return host->openDebug(libecap::LogVerbosity(verbosity.mask));
}

extern "C" void rust_shim_host_close_debug(libecap::host::Host *host, std::ostream* stream) {
    host->closeDebug(stream);
}

extern "C" void rust_shim_ostream_write(std::ostream *stream, char *buf, size_t length) {
    stream->write(buf, length);
}

extern "C" rust_shared_ptr_message rust_shim_host_new_request(libecap::host::Host *host) {
    return to_rust_shared_ptr_message(host->newRequest());
}

extern "C" rust_shared_ptr_message rust_shim_host_new_response(libecap::host::Host *host) {
    return to_rust_shared_ptr_message(host->newResponse());
}

extern "C" libecap::host::Host &rust_host() {
    return libecap::MyHost();
}

extern "C" rust_area rust_area_new() {
    return to_rust_area(libecap::Area());
}

extern "C" rust_area rust_area_new_slice(char *buf, size_t len) {
    return to_rust_area(libecap::Area::FromTempBuffer(buf, len));
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
	        libecap::host::Xaction *hostx;
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

extern "C" rust_area options_option(const libecap::Options *options, const rust_name* rname) {
    Must(options);
    // XXX: unavoidable copy due to Name's API
    libecap::Name name = from_rust_name(rname);
    libecap::Area area = options->option(name);
    return to_rust_area(area);
}

typedef void (*visitor_callback)(const rust_name, const rust_area, void*);

class NamedValueVisitorImpl: public libecap::NamedValueVisitor {
    public:
        NamedValueVisitorImpl(visitor_callback a_callback, void* an_extra):
            callback(a_callback), extra(an_extra) {};
        virtual void visit(const libecap::Name &name, const libecap::Area &value) {
            callback(to_rust_name(name), to_rust_area(value), extra);
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
    hostx = x;
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
    rust_area rarea = ::rust_xaction_ab_content(rust_xaction, hostx, offset, size);
    return from_rust_area(rarea);
}

void Adapter::Xaction::abContentShift(libecap::size_type size) {
    ::rust_xaction_ab_content_shift(rust_xaction, hostx, size);
}

void Adapter::Xaction::noteVbContentDone(bool atEnd) {
    ::rust_xaction_vb_content_done(rust_xaction, hostx, atEnd);
}

#define XACTION_METHOD_SHIM(rname__, cpp_name__) \
    void Adapter::Xaction::cpp_name__() { \
        ::rname__(rust_xaction, hostx); \
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


#define XACTION_METHOD_C_SHIM_INTERNAL(cpp_name__, c_name__) \
    extern "C" void c_name__(libecap::host::Xaction *xaction) { \
        xaction->cpp_name__();\
    }
#define XACTION_METHOD_C_SHIM(cpp_name__, c_name__) \
    XACTION_METHOD_C_SHIM_INTERNAL(cpp_name__, rust_shim_host_xaction_ ## c_name__)

XACTION_METHOD_C_SHIM(useVirgin, use_virgin);
XACTION_METHOD_C_SHIM(blockVirgin, block_virgin);
XACTION_METHOD_C_SHIM(adaptationAborted, adaptation_aborted);
XACTION_METHOD_C_SHIM(resume, resume);
XACTION_METHOD_C_SHIM(vbMake, vb_make);
XACTION_METHOD_C_SHIM(vbDiscard, vb_discard);
XACTION_METHOD_C_SHIM(vbPause, vb_pause);
XACTION_METHOD_C_SHIM(vbResume, vb_resume);
XACTION_METHOD_C_SHIM(vbMakeMore, vb_make_more);
XACTION_METHOD_C_SHIM(vbStopMaking, vb_stop_making);

extern "C" void rust_shim_host_xaction_adaptation_delayed(libecap::host::Xaction *xaction,
        const char* state, size_t len, double progress) {
    xaction->adaptationDelayed(libecap::Delay(std::string(state, len), progress));
}

extern "C" rust_area rust_shim_host_xaction_vb_content(libecap::host::Xaction *xaction, size_t offset, size_t size) {
    libecap::Area area = xaction->vbContent(offset, size);
    return to_rust_area(area);
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

extern "C" rust_shared_ptr_message rust_shim_message_clone(const libecap::Message *msg) {
    return to_rust_shared_ptr_message(msg->clone());
}

extern "C" const libecap::FirstLine *rust_shim_message_first_line(const libecap::Message *msg) {
    return &msg->firstLine();
}

extern "C" libecap::FirstLine *rust_shim_message_first_line_mut(libecap::Message *msg) {
    return &msg->firstLine();
}

extern "C" void rust_shim_message_add_body(libecap::Message *msg) {
    msg->addBody();
}

extern "C" const libecap::Body *rust_shim_message_body(const libecap::Message *msg) {
    return msg->body();
}

extern "C" libecap::Body *rust_shim_message_body_mut(libecap::Message *msg) {
    return msg->body();
}

extern "C" void rust_shim_message_add_trailer(libecap::Message *msg) {
    msg->addTrailer();
}

extern "C" const libecap::Header *rust_shim_message_trailer(const libecap::Message *msg) {
    return msg->trailer();
}

extern "C" libecap::Header *rust_shim_message_trailer_mut(libecap::Message *msg) {
    return msg->trailer();
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

extern "C" rust_version rust_shim_first_line_version(const libecap::FirstLine *first_line) {
    const auto version = first_line->version();
    return rust_version {
        majr: version.majr,
        minr: version.minr,
        micr: version.micr,
    };
}

extern "C" void rust_shim_first_line_set_version(libecap::FirstLine *first_line, const rust_version *version) {
    first_line->version(libecap::Version(version->majr, version->minr, version->micr));
}

extern "C" rust_name rust_shim_first_line_protocol(const libecap::FirstLine *first_line) {
    return to_rust_name(first_line->protocol());
}

extern "C" void rust_shim_first_line_set_protocol(libecap::FirstLine *first_line, const rust_name* protocol) {
    first_line->protocol(from_rust_name(protocol));
}

extern "C" bool rust_shim_header_has_any(const libecap::Header *header, const rust_name* name) {
    return header->hasAny(from_rust_name(name));
}

extern "C" rust_area rust_shim_header_value(const libecap::Header *header, const rust_name* name) {
    return to_rust_area(header->value(from_rust_name(name)));
}

extern "C" void rust_shim_header_add(libecap::Header *header, const rust_name* name, const rust_area* value) {
    auto area = libecap::Area(value->buf, value->size);
    const libecap::Name namev = from_rust_name(name);
    header->add(namev, area);
}

extern "C" void rust_shim_header_remove_any(libecap::Header *header, const rust_name* name) {
    header->removeAny(from_rust_name(name));
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
