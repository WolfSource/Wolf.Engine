#ifdef WOLF_MEDIA_FFMPEG

#include "w_av_packet.hpp"

using w_av_packet = wolf::media::ffmpeg::w_av_packet;

w_av_packet::w_av_packet() noexcept
    : _packet(nullptr) {}

w_av_packet::w_av_packet(AVPacket *p_av_packet) noexcept
    : _packet(p_av_packet) {}

w_av_packet::w_av_packet(w_av_packet &&p_other) noexcept {
  _move(std::move(p_other));
}

w_av_packet &w_av_packet::operator=(w_av_packet &&p_other) noexcept {
  _move(std::move(p_other));
  return *this;
}

void w_av_packet::_move(w_av_packet &&p_other) noexcept {
  if (this == &p_other)
    return;

  _release();

  this->_packet = std::exchange(p_other._packet, nullptr);
  if (p_other._own_data.size()) {
    this->_own_data.swap(p_other._own_data);
  }
}

w_av_packet::~w_av_packet() noexcept { _release(); }

boost::leaf::result<int> w_av_packet::init() noexcept {
  if (this->_packet != nullptr) {
    _release();
  }
  this->_packet = av_packet_alloc();
  if (this->_packet == nullptr) {
    return W_FAILURE(std::errc::not_enough_memory,
                     "could not allocate memory for avpacket");
  }
  return 0;
}

boost::leaf::result<int> w_av_packet::init(_In_ uint8_t *p_data,
                                            _In_ size_t p_data_len) noexcept {
  if (this->_packet != nullptr) {
    _release();
  }

  this->_packet = av_packet_alloc();
  if (this->_packet == nullptr) {
    return W_FAILURE(std::errc::not_enough_memory,
                     "could not allocate memory for avpacket");
  }

  this->_packet->data = p_data;
  this->_packet->size = gsl::narrow_cast<int>(p_data_len);

  return 0;
}

boost::leaf::result<int>
w_av_packet::init(_Inout_ std::vector<uint8_t> &&p_data) noexcept {
  if (p_data.size() == 0) {
    return {};
  }
  this->_own_data = std::move(p_data);
  return init(this->_own_data.data(), this->_own_data.size());
}

void w_av_packet::unref() noexcept { av_packet_unref(this->_packet); }

uint8_t *w_av_packet::get_data() const noexcept { return this->_packet->data; }

int w_av_packet::get_size() const noexcept { return this->_packet->size; }

int w_av_packet::get_stream_index() const noexcept {
  return this->_packet->stream_index;
}

void w_av_packet::_release() noexcept {
  if (this->_packet != nullptr) {
    av_packet_free(&this->_packet);
  }
  this->_own_data.clear();
}

#endif // WOLF_MEDIA_FFMPEG