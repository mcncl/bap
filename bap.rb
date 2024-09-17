# Formula for installing bap (Buildkite Agent Picker)
class Bap < Formula
  desc "Buildkite Agent Picker - 🚀 A Rust based Buildkite Agent Manager. "
  homepage "https://github.com/mcncl/bap"
  url "https://github.com/mcncl/bap/releases/download/v0.1.0/bap-0.1.0-arm64-apple-darwin.tar.gz"
  sha256 "44af0a5b679a69c9398e7cfecce72bcdc1e13804a6a9cf28f6c5be16cefef493"
  version "0.1.0"

  def install
    bin.install "bap"
  end

  test do
    system "#{bin}/bap", "--version"
  end
end
