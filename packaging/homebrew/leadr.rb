class Leadr < Formula
  desc "Leader-key inspired command runner"
  homepage "https://github.com/ll-nick/leadr"
  url "https://github.com/ll-nick/leadr/archive/refs/tags/v2.6.1.tar.gz"
  sha256 "aaf23e5f521911ab766876e96dd75422499db7f4d1316539bbfc36cbadabfa71"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/leadr", "--help"
  end
end
